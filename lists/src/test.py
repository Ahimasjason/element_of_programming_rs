from pandas import pandas as pd
from flask import flash, current_app
from simflow.simdams.exceptions import simdam_exceptions as excp
from simflow.db import db
import os
from simflow.simdams.models.loan_account import FileMappingMaster, PickledLoanWisePayout, CommonPayoutModel, PayoutModel, LoanAccountModel,\
                                               DealPoolSummary, PickledLoanWisePool,FileTypesMaster
from simflow.simdams.models.deal import DealMasterModel, DealChargeModel, ChargesTypeMaster
from simflow.simdams.models.borrower_master import BorrowerModel
from simflow.utilities.flash_exception import flash_exception
from sqlalchemy import and_,func, null
from sqlalchemy.orm import sessionmaker
from simflow.simdams.controllers.file_handler import FileHandler
from simflow.simdams.models.borrower_master import BorrowerTypeModel
import collections
import numpy as np
from simflow.simdams.simdams_ops.forms.form_validation import check_pan

ALLOWED_EXTENSIONS = set(['xlsx', 'csv'])
def allowed_file(filename):
    return '.' in filename and \
           filename.rsplit('.', 1)[1].lower() in ALLOWED_EXTENSIONS
def update_pool_file_etl(request,deal_id):
    # check if the post request has the file part
    if 'pool_file' not in request.files:
        raise excp.FilePartMissing(filename='pool_file')
    file = request.files['pool_file']
    # if user does not select file, browser also
    # submit an empty part without filename
    if not file or file.filename == '':
        raise excp.EmptyFilePart(filename='pool_file')

    #if the file type is not supported for upload
    if not allowed_file(file.filename):
        raise excp.FileTypeUnSupported(filename=file.filename)
    #if request does not form
    # if not request.form:  
    #     raise excp.FormMissingFromPostReq()
    else:
        form = request.form
        originator_id = int(form['originator'])
        product_category_id = form['product_category_id']
        product_id = form['product_id']
        file_ext = file.filename.rsplit('.', 1)[1].lower()
        # get file type id
        file_type_id= db.session.query(FileTypesMaster.id).filter(FileTypesMaster.code == '01').first()
        #get the column mapping from DB
        file_map = db.session.query(FileMappingMaster).\
            filter(and_(FileMappingMaster.file_type==file_type_id[0] , \
                FileMappingMaster.product_category_id==product_category_id ,\
                    FileMappingMaster.originator_id==originator_id, \
                        FileMappingMaster.product_id == product_id)).first()
        if not file_map : # check mapping present or not
            raise excp.RecordsNotFoundError(message = "Poll File mapping not found") # when mapping record not found
        col_map_list = file_map.col_mapping
        borro_map_list = file_map.borro_col_mapping
        if file_ext=='xlsx':
            df = pd.read_excel(file, dtype=str,header=None)
            count=len(df.index)
            # check xlsx file length,when len large then 50000 then refer csv file
            if count >= current_app.config['XLSX_MAX_LEN'] : 
                raise excp.LargeDataInXlFile(current_app.config['XLSX_MAX_LEN'])
        elif file_ext=='csv':
            #read the file with all columns as string
            df = pd.read_csv(file, dtype=str,header=None)
        else:
            raise excp.FileTypeUnSupported(filename=file.filename)
        if not len(df) > 0 :
            raise excp.RecordsNotFoundError(message = "Uploaded File has no records")
        col_name = df.iloc[0]
        if len(col_name) == 0 :
            # raise excp.HeadersNotFoundError()
            pass
        '''check duplicate header name'''
        dup_col_name = [item for item, count in collections.Counter(col_name).items() if count > 1]
        header_error_list = []
        if dup_col_name :
            dup_header_str ='  ,  '.join(map(str, dup_col_name))# list data convert in string with ',' for upload error file
            header_error_list.append({'key':'Duplicate header','value':dup_header_str})
        df = df.iloc[1: ]
        df.columns = col_name
        col_name =sorted( df.columns.tolist()) # sorted xl header name
        borro_data = []
        '''append all borrowers data in one list'''
        for i in borro_map_list :
            borro_data=borro_data+i['mapping_template']
        map_list = col_map_list + borro_data # combing column and borrower mapping data
        column_out_field_list = [] # list for column mapping editable fields related out field 
        borro_out_field_list = [] # list for borrower mapping editable fields related out field 
        borro_type_list = []# list for borrowereditable fields related borrower type
        df_xl_column_map = pd.DataFrame() # seprate df for column mapping editable fields
        df_xl_borro_map = pd.DataFrame() # seprate df for borrower mapping editable fields
        data_error_list = [] 
        '''check loan no header name '''
        loan_no_field_name = next(filter(lambda x: x["Out"]=='loan_no', col_map_list))# for find loan_no of in field sin column mapping
        if loan_no_field_name['In'] not in col_name :
            header_error_list.append({'key':'-','value':'Loan no header name not found'})
        col_map_in_fields = list(map(lambda x:x['In'], map_list)) # get all mapping In field name
        ''' find editable field heades name match with mapping in fields'''
        if not all(item in col_map_in_fields for item in col_name) :
            unmatch_header_list = list(set(col_name) - set(col_map_in_fields))
            if unmatch_header_list :
                unmatch_header_str ='  ,  '.join(map(str, unmatch_header_list))# list data convert in string with ',' for upload error file
                header_error_list.append({'key':'Invalid header','value':unmatch_header_str})
        if header_error_list :
            # raise excp.EditPoolFileHeaderError(header_error_list)
            pass
        df = df.sort_values(by =loan_no_field_name['In'] )
        ''' seprate list for column mapping data and borrower mapping data
            and validated pan no '''
        editable_data_list = list(filter(lambda x: x["In"] in col_name, map_list))
        editable_data_list = sorted(editable_data_list, key = lambda i: i['In']) 
        for i in  range(len(col_name)) :
            out_field_name = editable_data_list[i]['Out']
            mapping_type = editable_data_list[i]['MappingType']
            if mapping_type == 'columnMapping':
                df_xl_column_map[out_field_name] = df[col_name[i]]
            if mapping_type == 'borroMapping':
                if out_field_name == 'pan_no':
                    df_pan = pd.DataFrame()
                    df_pan['pan_error_status'], df_pan['pan_error_message']= zip(*df[col_name[i]].map(check_pan))
                    df_pan = df_pan.loc[df_pan['pan_error_status'] == False]
                    if len(df_pan) > 0 :
                        invalid_panno_str ='  ,  '.join(map(str, df[col_name[i]].tolist()))# list data convert in string with ',' for upload error file
                        data_error_list.append({'key':'Invalid PAN','value':invalid_panno_str})
                df_xl_borro_map[out_field_name] = df[col_name[i]] 
                borro_type_list.append(editable_data_list[i]['borro_type'])
        if len(df_xl_column_map) > 0 :
            '''check duplicate loan no data ,invalid loan no data from df '''
            if df[loan_no_field_name['In']].duplicated().any() :
                xl_dup_loan_no_list = df[df[loan_no_field_name['In']].duplicated()][loan_no_field_name['In']].tolist()
                if xl_dup_loan_no_list :
                    dup_loan_no_str ='  ,  '.join(map(str, xl_dup_loan_no_list))# list data convert in string with ',' for upload error file
                    data_error_list.append({'key':'duplicate loan no','value':dup_loan_no_str})
                    # raise excp.EditPoolFileDataError(data_error_list)
                    pass
            column_out_field_list = df_xl_column_map.columns.tolist()
            column_out_field_list.append('id')
            '''getting only editable field loan data from loans table'''
            #update query
            loan_data =  db.session.query(LoanAccountModel).\
                filter(and_(LoanAccountModel.deal_id==deal_id , \
                LoanAccountModel.product_category_id==product_category_id ,\
                    LoanAccountModel.originator==originator_id, \
                        LoanAccountModel.product_id == product_id,
                        LoanAccountModel.loan_no.in_(df[loan_no_field_name['In']]))).with_entities(*column_out_field_list).all()
            if not loan_data :
                raise excp.RecordsNotFoundError(message='Loan records not found')
            df_database_loan_data = pd.DataFrame(loan_data,columns = column_out_field_list)
            '''sort df from loan_no'''
            df_database_loan_data = df_database_loan_data.sort_values(by ='loan_no' )
            if len(df) != len(df_database_loan_data) :
                df_unmatch_data = list(set(df[loan_no_field_name['In']]) - set(df_database_loan_data['loan_no']) )    
                unmatch_loan_no_str ='  ,  '.join(map(str, df_unmatch_data))# list data convert in string with ',' for upload error file
                data_error_list.append({'key':'Invalid loan no','value':unmatch_loan_no_str})
            df_xl_column_map= df_xl_column_map.replace(["''", '',r'^\s*$','nan'],np.nan,regex=True)# converting empty string , spaces to nan
        '''any data error then raise this exception'''
        if data_error_list :
            # raise excp.EditPoolFileDataError(data_error_list)
            pass
        if len(df_xl_column_map) > 0 :
            '''merge two df and nan value replace with original database value
                on= 'loan_no' : replace data from comparing loan_no
                how = 'outer' : its required unique loan_no
                suffixes= ('_m', '_n') = create new column name with add string _m and _n'''
            combined_df = pd.merge(df_database_loan_data, df_xl_column_map, how = 'outer', on= 'loan_no', suffixes= ('_m', '_n'))
            for data in column_out_field_list  :
                if data not in ['loan_no','id'] :
                    '''where null column value then replace original database data related loanno'''
                    combined_df[data] = combined_df[str(data)+'_m'].where(combined_df[str(data)+'_n'].isnull(), combined_df[str(data)+'_n'])
                    combined_df = combined_df.drop([str(data)+'_m',str(data)+'_n'],axis=1)
            if len(combined_df) > 0 :
                db.session.bulk_update_mappings(LoanAccountModel, combined_df.to_dict('records'))
        if len(df_xl_borro_map) > 0 :
            borro_out_field_list = df_xl_borro_map.columns.tolist()
            borro_out_field_list.append('id')
            df_xl_borro_map['id'] = df_database_loan_data['id'].tolist()
            df_xl_borro_map= df_xl_borro_map.replace(["''", '',r'^\s*$','nan'],np.nan,regex=True)# converting empty string , spaces to nan
            #update query
            borro_data =  db.session.query(BorrowerModel).\
                filter(and_(BorrowerModel.borrower_type_id.in_(borro_type_list) , \
                BorrowerModel.loan_id.in_(df_database_loan_data['id'].tolist()),\
                    )).with_entities(*borro_out_field_list).all()
            df_database_borro_data = pd.DataFrame(borro_data,columns = borro_out_field_list)
            if not borro_data :
                raise excp.RecordsNotFoundError(message='Borrower records not found')
            combined_df = pd.merge(df_database_borro_data, df_xl_borro_map, how = 'outer', on= 'id', suffixes= ('_m', '_n'))
            for data in borro_out_field_list  :
                if data not in ['id'] :
                    combined_df[data] = combined_df[str(data)+'_m'].where(combined_df[str(data)+'_n'].isnull(), combined_df[str(data)+'_n'])
                    del combined_df[str(data)+'_m']
                    del combined_df[str(data)+'_n']
            if len(combined_df) > 0 :
                db.session.bulk_update_mappings(BorrowerModel, combined_df.to_dict('records'))
        db.session.commit()
            