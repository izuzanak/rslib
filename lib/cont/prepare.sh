
CLIB_BUILD_DIR="/home/jirka/folder/work/git/clib/build"

export RUSTFLAGS=\
'--cfg OPTION_TO_STRING="ENABLED" '\
'--cfg OPTION_TO_JSON="ENABLED" '\
" -L $CLIB_BUILD_DIR"

export LD_LIBRARY_PATH="$CLIB_BUILD_DIR"

