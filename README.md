# practice_github_and_python
* Githubを練習するためのリポジトリ
  * Githubを使い慣れている人は教えてください
  * 管理している人も初心者なので
* 特に理由はないがpythonの練習にも
* 詰まったところを検索してこのREADME.mdに追記
  * エラーの内容などを書いてどうやって解決したかを記述
  * 検索して参考にしたURLなどなど
* pythonのファイルなども追加しても良い
  * hello.pyに新しい関数の追加
  * 新しく.pyのもしくは.ipynbなどのファイルを追加
* mainのbranchではなく**develop**のbranchにpush
* markdown表記に関して  
  URL: https://docs.github.com/ja/github/writing-on-github/basic-writing-and-formatting-syntax
## Gitをインストールしていない人はインストールから
URL: https://git-scm.com/downloads

## Githubのアカウントを持っていない人はアカウントを作るところから
URL: https://github.com/  
Sign upをクリック

## とりあえずpythonで(飛ばしても良い)
### 1. pythonの環境構築から($は入力しない)  
$  python3 -m venv dir_name  
注意) dir_nameはなんでも良い  
ex) $ python3 -m venv my_dir  
### 2. 環境をアクティブにする  
$  cd dir_name  
$  source /bin/activate  
注意) 上記のはbash/zshなのでwindowsの人は気を付ける  
参考) url: https://docs.python.org/ja/3/library/venv.html  
$ mkdir src && cd src

## リポジトリのクローン
$ git clone repo_URL   
ex) git clone https://github.com/TyswSh/practice_github_and_python.git  
$ cd practice_github_and_python

## Gitの使い方
参考) URL:https://qiita.com/nnahito/items/565f8755e70c51532459
### 1.originの設定
$ git remote add origin repo_URL  
ex) git  remote add origin https://github.com/TyswSh/practice_github_and_python.git
### 2.基本は**add/commit/push**の3つを利用
$ git add file_name  
$ git commit -m "Explanatory text"  
$ git push origin develop  

この3つがこの先何度も打ち込むコマンド
### 3.branchの変更
* 今いるbranch  
$ git branch
* branchの変更  
$ git branch branch_name  
  注意) branch_nameを既に作っていたら飛ばす  
$ git checkout branch_name  
  注意) mainではなくdevelopを作成してそこで変更を加えていく  

### 変更を加えて2をひたすら回す
addはこまめに  
commitはタスクを終えたら  
* 新しく関数を追加
* バグの修正
* 修正や使用の変更  

pushはcommitしたらすぐに  
ただし，pushしたらすぐにマージされるわけではないので気を付ける  
また，他の人がpush(変更)した場合はfetch  
$ git fetch  
$ git branch  
$ git merge develop
