if [ -z ${DEVDIR+x} ];
then
    return
fi

startdir=${PWD}
rm -rf $DEVDIR/_DEL_*

# ///////////////////////
echo "--> Creating dummy repos."
cd $DEVDIR
repodir="_DEL_new_empty"
mkdir $repodir
cd $repodir

git init && git commit --allow-empty -m "init."

cd $startdir
# ///////////////////////

# ///////////////////////
echo "--> Creating dummy repos."
cd $DEVDIR
repodir="_DEL_new_w_branches"
mkdir $repodir
cd $repodir

git init && git commit --allow-empty -m "init."
git branch dev master
git branch feature master
git branch hotfix master

cd $startdir
# ///////////////////////

# ///////////////////////
echo "--> Creating dummy repos."
cd $DEVDIR
repodir="_DEL_new_w_branches-gc"
mkdir $repodir
cd $repodir

git init && git commit --allow-empty -m "init."
git branch dev master
git branch feature master
git branch hotfix master
git gc --aggressive --prune=all

cd $startdir
# ///////////////////////

# ///////////////////////
echo "--> Creating dummy repos."
cd $DEVDIR
repodir="_DEL_new_w_bad_status"
mkdir $repodir
cd $repodir

git init && git commit --allow-empty -m "init."
touch file_1 file_2 file_3
git add . && git commit -m 'added files.'
echo "stuff" >> file_1 && git add file_1
echo "stuff" >> file_2
rm file_3
touch file_4

cd $startdir
# ///////////////////////

# ///////////////////////
echo "--> Creating dummy repos."
cd $DEVDIR
repodir="_DEL_not_master_bad_status"
mkdir $repodir
cd $repodir

git init && git commit --allow-empty -m "init."
git checkout -b "wip-branch"
touch file_1 file_2 file_3
git add . && git commit -m 'added files.'
echo "stuff" >> file_1 && git add file_1
echo "stuff" >> file_2
rm file_3
touch file_4

cd $startdir
# ///////////////////////
