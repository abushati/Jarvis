import hashlib
import os
# import magic
import requests

f = open('/Users/arvidbushati/Desktop/Resume.docx', 'rb'):
f_bytes = f.read()    
# mime = magic.Magic(mime=True)
# mimeType = mime.from_file(f.name)
filename = os.path.splitext(file_name)[0]
md5 = hashlib.md5(f_bytes).hexdigest()

a = {'fileName':'test.docx','md5':md5}
# a = {'fileName':'test.docx'}
re = requests.post('http://127.0.0.1:8080/upload_file', json=a)


# requests.post('http://127.0.0.1:8080/upload_file_data/43', data=b)