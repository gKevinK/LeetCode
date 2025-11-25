# coding: utf-8

import json
import os
import re
import svg


langs = {
    'cpp': ['#f34b7d', 'C++'],
    'rs': ['#dea584', 'Rust'],
    'sh': ['#89e051', 'Shell'],
    'sql': ['#e38c00', 'SQL'],
}


problem_info = {
    'num': 0,
    'paid_only': []
}
if os.path.isfile('.info'):
    print('Read problem info')
    with open('.info', encoding='utf-8') as f:
        problem_info = json.load(f)
else:
    print('Problem info not found')


print('Generate SVG')

GROUP_SIZE = 500
COLUMNS = 50

elements = []
elements.append(svg.Style(text='''.g { font: normal 12px sans-serif; fill: #999999 }'''))


for i, lang in enumerate(langs):
    elements.append(svg.Rect(x=11 + i * 60, y=4,
        width=8, height=8,
        rx=1, ry=1,
        fill=langs[lang][0]))
    elements.append(svg.Text(x=24 + i * 60, y=12, class_=['g'], text=langs[lang][1]))

elements.append(svg.Rect(x=11 + len(langs) * 60, y=4,
    width=8, height=8,
    # rx=1, ry=1,
    fill='#999999'))
elements.append(svg.Text(x=24 + len(langs) * 60, y=12, class_=['g'], text='More than one language'))


y_base = 20
for group in range((problem_info['num'] + GROUP_SIZE - 2) // GROUP_SIZE):
    start = group * GROUP_SIZE + 1
    end = (group + 1) * GROUP_SIZE
    dn = f'{start:04}-{end:04}'
    accepted = {}
    for fn in os.listdir(dn):
        m = re.match(r'(\d{4})_(.+)\.(.+)', fn)
        fid = int(m.group(1))
        if fid not in accepted:
            accepted[fid] = []
        accepted[fid].append(m.group(3))

    elements.append(svg.Text(x=10, y=y_base + 16, class_=['g'], text=f'{start}-{end}'))
    for i in range(GROUP_SIZE):
        if start + i >= problem_info['num']:
            break
        if (start + i) in problem_info['paid_only']:
            continue
        if start + i in accepted:
            fill = langs[accepted[start + i][0]][0]
            if len(accepted[start + i]) > 1:
                fill = '#999999'
            elements.append(svg.Rect(
                x=10 * (i % COLUMNS) + 11, y=10 * (i // COLUMNS) + y_base + 20,
                width=8, height=8,
                # rx=1, ry=1,
                fill=fill,
            ))
        else:
            elements.append(svg.Rect(
                x=10 * (i % COLUMNS) + 11, y=10 * (i // COLUMNS) + y_base + 20,
                width=8, height=8,
                # rx=1, ry=1,
                fill='#999999',
                fill_opacity=0.2,
            ))
    y_base += 120


canvas = svg.SVG(
    width=520,
    height=y_base + 10,
    elements=elements,
)


os.makedirs('output', exist_ok=True)
with open('output/progress.svg', 'w') as f:
    f.write(canvas.as_str())
