def count_char(c,i,code):
    n = 0
    while i<len(code):
        if code[i] == c:
            # 同じ文字ならnを増やす
            n+=1
            i+=1
        elif code[i] in '+-><[],.':
            # 別の記号が来たら終わる
            break
        else:
            # 無意味な文字列は飛ばす
            i+=1
    return n,i

def parse(code,i=0,depth=0):
    # iは常に読んでない文字を指す
    ast = []
    cmd_map = {
            '+': ('+',+1),
            '-': ('+',-1),
            '>': ('>',+1),
            '<': ('>',-1),
            '.': ('.',+1),
            ',': (',',+1),
    }
    
    while i<len(code):
        c = code[i]

        if c in '+-><,.':
            # これらの文字は文字数をカウントする
            n,i = count_char(c,i,code)
            ast.append((cmd_map[c][0],cmd_map[c][1] * n))
        elif c=='[':
            # ループの最初が出たら再帰呼出ししてうんぬんする
            ast_leaf,i = parse(code,i+1,depth+1)
            ast.append(ast_leaf)       
        elif c==']':
            # ループの外部で]が出たら]が多すぎる
            if depth == 0:
                raise RuntimeError('too many ]')
            else:
                return ast,i+1
        else:
            i += 1

    if depth != 0:
        raise RuntimeError('too many [')
    else:
        # depth==0のときだけastを返す
        return ast

def optimize(ast):
    return [optimize_core(cmd)for cmd in ast]

def optimize_core(cmd):
    if isinstance(cmd,list):
        # ループの場合はそれ用の最適化をかける
        return optimize_loop(cmd)
    else:
        # 普通のコマンドの場合はもう最適化かけない
        return cmd

def optimize_loop(ast):
    ast = optimize(ast)
    if len(ast) == 1:
        return optimize_assign_zero(ast)
    else:
        return optimize_mult_add(ast)

def optimize_assign_zero(ast):
    # len(ast) == 0
    # [-]をassign 0にする
    cmd = ast[0]

    # ループが中にあったらだめ
    if isinstance(cmd,list):
        return ast
    
    c,n = cmd

    # (-,1)や(+,1)のときはassign 0に変更
    if c == '+' and abs(n) == 1:
        return ('=',0)
    else:
        return ast
            
def optimize_mult_add(ast):
    # [->>>++<<<]を最適化して
    # mem[ptr+3] += 2*mem[ptr]にする

    # loopが入ってたらだめ
    if any([isinstance(i,list) for i in ast]):
        return ast

    symbols = [c[0] for c in ast]
    
    # add-mem,add-ptrのみでなければだめ
    if not all(['+' == i or '>' == i for i in symbols]):
        return ast
    
    last_ptr,change = pointer_movement(ast)

    # ループは常に同じptrを見ていなければだめ
    if last_ptr != 0:
        return ast

    # 条件に使う値は1ずつ減らないとだめ
    if 0 not in change.keys() or change[0] != -1:
        return ast
    
    return ('x',[(k,change[k])for k in change])

def pointer_movement(ast):
    ptr = 0
    change = {}
    for cmd in ast:
        c,n = cmd
        if c == '+':
            change[ptr] = change[ptr] + n if ptr in change.keys() else n
        elif c == '>':
            ptr += n
        else:
            raise RuntimeError('unkown %s' % c)
    return ptr,change

def run(code):
    ast = parse(code)
    ast = optimize(ast)
    mem = [0]*65536
    ptr = 0
    core_run(ast,ptr,mem)

def core_run(ast,ptr,mem,is_loop=False):
    # とりあえず一通り実行し，ループする場合は再帰呼出し
    for cmd in ast:
        # リストの中にリストがあったらループ
        # mem[ptr]!=0なら実行する
        if isinstance(cmd,list):
            ptr = core_run(cmd,ptr,mem,True) if mem[ptr] != 0 else ptr
            continue

        # 各コマンドをn回実行する
        c,n = cmd
        if c == '+':
            mem[ptr]=(mem[ptr]+n)%256
        elif c == '>':
            ptr += n
            check_ptr(ptr,mem)
        elif c == '.':
            [print('%c' % chr(mem[ptr]),end='') for _ in range(n)]
        elif c == ',':
            raise RuntimeError('\',\' is not defined')
        elif c == '=':
            mem[ptr] = n
        elif c == 'x':
            times = mem[ptr]
            
            if times == 0:
                continue
            for (move,val) in n:
                check_ptr(ptr+move,mem)
                mem[ptr+move] = (mem[ptr+move] + times * val) % 256
        else:
            raise RuntimeError('something wrong:%s',c)


    if is_loop == False or mem[ptr] == 0:
        return ptr
    else:
        return core_run(ast,ptr,mem,True)

def check_ptr(ptr,mem):
    if ptr < 0 or ptr >= len(mem):
        raise RuntimeError('ptr is out of range: %d' % (ptr) )

def usage():
    print('python3 brainfuck.py mandelbrot.bf')

def main():
    import sys
    
    if len(sys.argv) != 2:
        usage()
    else:
        with open(sys.argv[1],'r') as f:
            code = ''.join(f.readlines())
        run(code)

if __name__=='__main__':
    main()
