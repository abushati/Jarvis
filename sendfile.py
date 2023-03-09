import hashlib
import os
# import magic
import requests
file_name = '/Users/arvidbushati/Desktop/Resume.docx'

f = open(file_name, 'rb')
f_bytes = f.read()    

filename = os.path.splitext(file_name)[0]
md5 = hashlib.md5(f_bytes).hexdigest()
a = {'fileName':'test.docx','md5':md5}

re = requests.post('http://127.0.0.1:8080/upload_file', json=a)
print(re.text)


re = requests.post('http://127.0.0.1:8080/upload_file_data/{}'.format(re.text), data=str(f_bytes))
print(re.text)