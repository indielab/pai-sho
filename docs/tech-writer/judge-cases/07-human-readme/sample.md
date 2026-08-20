`brew trust` records trust for your user. `sudo brew services` is the one root
use Homebrew allows (it loads a launchd service, runs no build scripts); `sudo
brew trust` and `sudo brew install` are refused, and that refusal is correct.
`--preserve-env=XDG_CONFIG_HOME` matters only if you set `XDG_CONFIG_HOME`: plain
`sudo` strips it, so brew looks for your trust file under `$HOME/.homebrew`
instead of your real config home and refuses the tap. Preserving it points brew
back where `brew trust` wrote. (Harmless if you don't set `XDG_CONFIG_HOME`.)
