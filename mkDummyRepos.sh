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
# ///////////////////////
cd $DEVDIR && repodir="__DEL_WIP_REPO_05" && mkdir $repodir && cd $repodir
git init && git commit --allow-empty -m "init."
git checkout -b dev && git checkout -b hotfix && git checkout -b rnd && git checkout -b ticket
touch untracked_file
git add untracked_file && git commit -m 'commit 1'
echo content > untracked_file
cd $startdir
