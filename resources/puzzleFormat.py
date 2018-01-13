import re

puzzle_pattern = re.compile(r'Grid \d\d\n((?:\d{9}\n){9})')

with open('puzzles_euler.txt', 'r') as f:
    contents = f.read()

matches = re.findall(puzzle_pattern, contents)
new_content = ''
for match in matches:
    new_content += match.replace('\n','') + '\n'

with open('puzzles.txt','w') as f:
    f.write(new_content)
