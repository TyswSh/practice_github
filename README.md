# Practice Github
* Githubを練習するためのリポジトリ
  * Githubを使い慣れている人は教えてください
  * 管理している人も初心者なので
* 特に理由はないがpythonの練習にも
* 詰まったところを検索してこのREADME.mdに追記
  * エラーの内容などを書いてどうやって解決したかを記述
  * 検索して参考にしたURLなどなど
* pythonや別の言語のファイルなども追加しても良い
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
### Gitの設定
参考) https://qiita.com/tetsu-upstr/items/e72147250701cf30ee72  
Gitなどはターミナル(git bashなど)から基本的に実行  
global設定  
* 改行設定  
$ git config --global core.autocrlf false  
* ユーザー設定  
$ git config --global user.name "John Doe"
* メール設定  
$ git config --global user.email johndoe@example.com

## ssh keyの生成とGithubへの登録
ターミナルからGithubにアクセス可能に  
よくあるミスでssh keyの設定をし忘れる，または上手く登録できない場合がある  
### ターミナルからssh keyの生成  
home もしくは c\users\user_name\へ移動  
$ cd  
.sshに移動  
$ cd ~/.ssh  
もしcd ~/.sshがエラーが出たら  
$ mkdir ~/.ssh
秘密鍵と公開鍵の生成  
参考) https://qiita.com/suthio/items/2760e4cff0e185fe2db9  
$ ssh-keygen -t rsa -b 4096 -C "your_email@example.com"  
いろいろ聞かれるが基本的に変更を加えたくなければそのままエンター  
パスフレーズを入力  
$ Enter passphrase (empty for no passphrase): passphrase  
ssh keyの生成完了  
念のためにlsもしくはdirでid_ras.pubとid_rsaの確認
* id_rsa.pub は誰かに教えても大丈夫  
* d_rsa は絶対誰にも教えてはいけない  

### 公開鍵をGithubへ登録
テキストエディタで~/.sshのid_ras.pubを開きコピー  
[github](https://github.com/)のsettingsへ移動
SSH and GPG keysへ移動  
SSH keysのNew SSH keyをクリック  
titleは自分のパソコンからと分かるようにして，コピーしたのをKeyへペースト
ターミナルを利用して接続確認  
$ ssh -T git@github.com  
Hi (account名)! You've successfully authenticated, but GitHub does not provide shell access.  
これが返ってきたら成功  
エラーが出たら公開鍵の生成で変な引数を入れている可能性があるので，もう一回やってみる
### .originの設定
$ git remote add origin repo_URL  
ex) git  remote add origin https://github.com/TyswSh/practice_github_and_python.git
### .基本は**add/commit/push**の3つを利用
$ git add file_name  
$ git commit -m "Explanatory text"  
$ git push origin develop  

この3つがこの先何度も打ち込むコマンド
### .branchの変更
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
