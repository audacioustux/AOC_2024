source $ZDOTDIR/.zprofile

OMZ="$PROFILE_ROOT/share/oh-my-zsh"

ZSH_THEME="robbyrussell"

plugins=(
	git
	fzf
	asdf
	zoxide
	kubectl
	direnv
)

source $OMZ/oh-my-zsh.sh

source $PROFILE_ROOT/share/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh
source $PROFILE_ROOT/share/zsh-autosuggestions/zsh-autosuggestions.zsh
