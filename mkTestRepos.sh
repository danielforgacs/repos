here=$PWD
td=$(mktemp -d --suffix "__repos_test")
export DEVDIR_ORIGINAL=$DEVDIR
export DEVDIR=$td
echo $td

mkdir $td/no_commit && cd $td/no_commit && \
    git init

mkdir $td/just_init && cd $td/just_init && \
    git init && \
    git commit --allow-empty -m 'Init.'

mkdir $td/new_file && cd $td/new_file && \
    git init && \
    touch untracked_new_file

mkdir $td/new_file_staged && cd $td/new_file_staged && \
    git init && \
    git commit --allow-empty -m 'Init.' && \
    echo "nothing." > staged_new_file && \
    git add staged_new_file

mkdir $td/modified_files && cd $td/modified_files && \
    git init && \
    git commit --allow-empty -m 'Init.' && \
    echo "nothing." > some_file && \
    git add some_file && \
    git commit -m '1st change' && \
    echo "1st update." >> some_file

cd $here
