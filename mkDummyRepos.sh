if [ -z ${DEVDIR+x} ];
then
    return
fi

startdir=${PWD}

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
