ZSH_COMPDUMP="${ZSH_CACHE_DIR}/zcompdump-${SHORT_HOST}-${ZSH_VERSION}"

. <(devbox shellenv -c $WORKSPACE_FOLDER --init-hook --install)
export PROFILE_ROOT="$DEVBOX_PROJECT_ROOT/.devbox/nix/profile/default"
