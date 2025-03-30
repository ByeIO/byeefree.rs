# 转换package.toml到package.json文件
# pip install toml -i https://mirrors.aliyun.com/pypi/simple/
# python -c "import toml, json; data = toml.load(open('package.toml')); json.dump(data, open('package.json', 'w'), indent=2)"
import toml, json; 
data = toml.load(open('package.toml')); 
json.dump(data, open('package.json', 'w'), indent=2)
