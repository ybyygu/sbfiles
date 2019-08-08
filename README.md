
# 程序目的

方便本地和远程(SSH)主机间小文件的交换, 即

1.  复制本地文件 &#x2013;> 远程主机命令行下当前目录粘贴
2.  复制远程主机中的文件 &#x2013;> 本地粘贴

这个小玩意不是为了取代scp/sftp之类的文件传输工具的, 而是为了在以下几种情况下更方便地进行文件交换:

1.  直接将本地文件送到远程主机命令行所在的当前目录下, 或将命令行目录下的文件直接取回本地. 省去scp和sftp切换目录的麻烦.
2.  特别是当目标文件所在的远程主机位于重重包裹中, 没有办法直接来scp或
    sftp时, 这个小玩意用起来可能会更方便.


# 实现原理

1.  将多个文件打包为tar.gz文件.
2.  将tar.gz文件编码为base64, 实现纯文本化. 这里借鉴了邮件收发时对附件的处理.
3.  通过字符终端的屏幕scrollback buffer来实现本地和远程文件的双向交换.

由于是通过终端屏幕来中转, 因此, 一次所能处理的文件大小就受限于终端最大能显示的行数. 这个数值(lines of scrollback)是可手动设置的. 不同的终端默认设置不同,
mintty里是10000行, putty里只有200行. 设大一些, 这样用来处理几百KB的文件(zip压缩后)不在话下.


# 使用步骤

sbfiles配合tmux使用, 最便利.


## 将远程主机文件复制到本地

1.  在tmux中ssh到远程主机(可任意嵌套).
2.  在远程主机上使用sbfiles编码文件:
    
        sbfiles encode file-or-dir
3.  在本地某个目录下执行如下命令(outside tmux):
    
        tmux capture-pane -S - -E - -p|sbfiles decode


## 将本地文件复制到远程主机

1.  在本地主机上使用sbfiles编码文件:
    
        sbfiles encode file-or-dir
2.  复制tmux屏幕内容至buffer
    
        ctrl-b :capture-pane -S -
3.  在tmux中ssh到远程主机(可任意嵌套).
4.  在远程主机某个目录下执行如下命令(inside tmux):
    
        sbfiles decode
    
    将tmux之前保存的buffer贴入后, 按ctrl-d确认即可:
    
        ctrl-b :paste-buffer

