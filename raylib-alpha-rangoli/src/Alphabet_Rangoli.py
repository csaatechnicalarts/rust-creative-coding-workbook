import string

def print_rangoli(n: int) -> None:
    rangoli_lines = []

    for i in range(n-2, -2, -1):
        rangoli_lines.append(print_line(n-1, i))

    max_width = len(rangoli_lines[-1])

    for line in rangoli_lines:
        print(line.center(max_width, '-'))

    for m in range(len(rangoli_lines)-2, -1, -1):
        print(rangoli_lines[m].center(max_width, '-'))

def print_line(n: int, m: int) -> str:
    lowercase_alpha = string.ascii_lowercase
    list_la = (lowercase_alpha)

    r_line = []
    i = n
    while i > m:
        r_line.append(list_la[i])
        i -= 1

    for j in range(i+2, n+1):
        r_line.append(list_la[j])

    return '-'.join(r_line)

if __name__ == '__main__':
    n = int(input())
    print_rangoli(n)