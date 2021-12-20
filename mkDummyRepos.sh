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
# ///////////////////////
cd $DEVDIR && repodir="__DEL_WIP_REPO_06" && mkdir $repodir && cd $repodir
git init && git commit --allow-empty -m "init."
git branch branch-1 && git branch branch-2 && git branch branch-3 && git branch branch-4 && git branch branch-5 && git branch branch-6 && git branch branch-7 && git branch branch-8 && git branch branch-9 && git branch branch-10 && git branch branch-11 && git branch branch-12 && git branch branch-13 && git branch branch-14
cd $startdir
