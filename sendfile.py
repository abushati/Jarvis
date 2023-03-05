with open('/Users/arvidbushati/Desktop/Resume.docx', 'rb') as f:
    b = f.read()

a = {'fileName':'test.docx','fileData':b}
# a = {'fileName':'test.docx'}
requests.post('http://127.0.0.1:8080/upload_file', json=a)



requests.post('http://127.0.0.1:8080/upload_file_data/43', data=b)