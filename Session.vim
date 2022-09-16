let SessionLoad = 1
let s:so_save = &g:so | let s:siso_save = &g:siso | setg so=0 siso=0 | setl so=-1 siso=-1
let v:this_session=expand("<sfile>:p")
silent only
silent tabonly
cd ~/mp3-tagger-rust
if expand('%') == '' && !&modified && line('$') <= 1 && getline(1) == ''
  let s:wipebuf = bufnr('%')
endif
let s:shortmess_save = &shortmess
if &shortmess =~ 'A'
  set shortmess=aoOA
else
  set shortmess=aoO
endif
badd +1 ~/mp3-tagger-rust
badd +0 src/main.rs
badd +1 terminal\ fish
badd +6 term://~/mp3-tagger-rust//3522:fish
badd +0 term://~/mp3-tagger-rust//4005:fish
argglobal
%argdel
$argadd ~/mp3-tagger-rust
edit NetrwTreeListing
let s:save_splitbelow = &splitbelow
let s:save_splitright = &splitright
set splitbelow splitright
wincmd _ | wincmd |
split
1wincmd k
wincmd _ | wincmd |
vsplit
1wincmd h
wincmd w
wincmd w
let &splitbelow = s:save_splitbelow
let &splitright = s:save_splitright
wincmd t
let s:save_winminheight = &winminheight
let s:save_winminwidth = &winminwidth
set winminheight=0
set winheight=1
set winminwidth=0
set winwidth=1
exe '1resize ' . ((&lines * 58 + 33) / 66)
exe 'vert 1resize ' . ((&columns * 20 + 64) / 129)
exe '2resize ' . ((&lines * 58 + 33) / 66)
exe 'vert 2resize ' . ((&columns * 108 + 64) / 129)
exe '3resize ' . ((&lines * 5 + 33) / 66)
argglobal
balt terminal\ fish
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
let &fdl = &fdl
let s:l = 5 - ((4 * winheight(0) + 29) / 58)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 5
normal! 0
lcd ~/mp3-tagger-rust
wincmd w
argglobal
if bufexists(fnamemodify("~/mp3-tagger-rust/src/main.rs", ":p")) | buffer ~/mp3-tagger-rust/src/main.rs | else | edit ~/mp3-tagger-rust/src/main.rs | endif
if &buftype ==# 'terminal'
  silent file ~/mp3-tagger-rust/src/main.rs
endif
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
let &fdl = &fdl
let s:l = 2 - ((1 * winheight(0) + 29) / 58)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 2
normal! 0
lcd ~/mp3-tagger-rust
wincmd w
argglobal
if bufexists(fnamemodify("term://~/mp3-tagger-rust//4005:fish", ":p")) | buffer term://~/mp3-tagger-rust//4005:fish | else | edit term://~/mp3-tagger-rust//4005:fish | endif
if &buftype ==# 'terminal'
  silent file term://~/mp3-tagger-rust//4005:fish
endif
balt ~/mp3-tagger-rust/terminal\ fish
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
let s:l = 1 - ((0 * winheight(0) + 2) / 5)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 1
normal! 0
lcd ~/mp3-tagger-rust
wincmd w
2wincmd w
exe '1resize ' . ((&lines * 58 + 33) / 66)
exe 'vert 1resize ' . ((&columns * 20 + 64) / 129)
exe '2resize ' . ((&lines * 58 + 33) / 66)
exe 'vert 2resize ' . ((&columns * 108 + 64) / 129)
exe '3resize ' . ((&lines * 5 + 33) / 66)
tabnext 1
if exists('s:wipebuf') && len(win_findbuf(s:wipebuf)) == 0 && getbufvar(s:wipebuf, '&buftype') isnot# 'terminal'
  silent exe 'bwipe ' . s:wipebuf
endif
unlet! s:wipebuf
set winheight=1 winwidth=20
let &shortmess = s:shortmess_save
let &winminheight = s:save_winminheight
let &winminwidth = s:save_winminwidth
let s:sx = expand("<sfile>:p:r")."x.vim"
if filereadable(s:sx)
  exe "source " . fnameescape(s:sx)
endif
let &g:so = s:so_save | let &g:siso = s:siso_save
set hlsearch
nohlsearch
doautoall SessionLoadPost
unlet SessionLoad
" vim: set ft=vim :
