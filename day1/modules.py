
def calc(mw):
    return mw / 3 - 2

def rec(acc, last):
    n = calc(last)
    if n <= 0:
        return acc
    return rec(acc + n, n)

def main():
    print("12: %d" % calc(12))
    print("14: %d" % calc(14))
    print("1969: %d" % calc(1969))
    print("100756: %d" % calc(100756))
    print("r14: %d" % rec(calc(14), calc(14)))
    print("r100756: %d" % rec(calc(100756), calc(100756)))

    f = open("modules.txt", "r")
    acc = 0
    for l in f:
        l = l.strip()
        i = int(l)
        m = calc(i)
        wtax = rec(m, m)
        acc += wtax
    print("Acc: %d" % acc)

main()
