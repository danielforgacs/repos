startdir=${PWD}
devdir=$startdir/__DEL_repos
export DEVDIR=$devdir
rm -rf $DEVDIR
mkdir $DEVDIR

# ///////////////////////
cd $DEVDIR && repodir="__DEL_WIP_REPO_01" && mkdir $repodir && cd $repodir
git init && git commit --allow-empty -m "init."
cd $startdir
# ///////////////////////
# ///////////////////////
cd $DEVDIR && repodir="__DEL_WIP_REPO_02" && mkdir $repodir && cd $repodir
git init && git commit --allow-empty -m "init."
git checkout -b dev && git checkout -b hotfix && git checkout -b rnd && git checkout -b ticket
cd $startdir
# ///////////////////////
cd $DEVDIR && repodir="__DEL_WIP_REPO_03" && mkdir $repodir && cd $repodir
git init && git commit --allow-empty -m "init."
touch untracked_file
cd $startdir
# ///////////////////////
cd $DEVDIR && repodir="__DEL_WIP_REPO_04" && mkdir $repodir && cd $repodir
git init && git commit --allow-empty -m "init."
touch untracked_file
git add untracked_file && git commit -m 'commit 1'
echo content > untracked_file
cd $startdir






# # ///////////////////////
# cd $DEVDIR
# repodir="__DEL_WIP_REPO_0002"
# mkdir $repodir
# cd $repodir
# 
# git init && git commit --allow-empty -m "init."
# git branch dev master
# git branch feature master
# git branch hotfix master
# 
# cd $startdir
# # ///////////////////////
# cd $DEVDIR
# repodir="__DEL_WIP_REPO_0003"
# mkdir $repodir
# cd $repodir
# 
# git init && git commit --allow-empty -m "init."
# git branch dev master
# git branch feature master
# git branch hotfix master
# git gc --aggressive --prune=all
# 
# cd $startdir
# # ///////////////////////
# cd $DEVDIR
# repodir="__DEL_WIP_REPO_0004"
# mkdir $repodir
# cd $repodir
# 
# git init && git commit --allow-empty -m "init."
# touch file_1 file_2 file_3
# git add . && git commit -m 'added files.'
# echo "stuff" >> file_1 && git add file_1
# echo "stuff" >> file_2
# rm file_3
# touch file_4
# 
# cd $startdir
# # ///////////////////////
# cd $DEVDIR
# repodir="__DEL_WIP_REPO_0005"
# mkdir $repodir
# cd $repodir
# 
# git init && git commit --allow-empty -m "init."
# git checkout -b "wip-branch"
# touch file_1 file_2 file_3
# git add . && git commit -m 'added files.'
# echo "stuff" >> file_1 && git add file_1
# echo "stuff" >> file_2
# rm file_3
# touch file_4
# 
# cd $startdir
# # ///////////////////////
# cd $DEVDIR
# repodir="__DEL_WIP_REPO_0006"
# mkdir $repodir
# cd $repodir
# 
# git init && git commit --allow-empty -m "init."
# git checkout -b "wip-branch"
# touch file_1 file_2 file_3
# git add . && git commit -m 'added files.'
# 
# cd $startdir
# # ///////////////////////
