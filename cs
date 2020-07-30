#!/usr/bin/env bash
set -euo pipefail

cmd=$(basename "$0")

helpmsg() {
	cat << EOF
Usage: $cmd COMMAND [ARGS]...

General commands:
    help      Display this message and exit
    task      Set up a given task environment (requires \$COURSE)
    remote    Run a command on the CSE servers

Working on a given task (requires \$COURSE and \$TASK)
    fetch     Download provided material
    autotest  Run specified autotests
    give      Submit work
EOF
	exit
}

die() {
	echo "$cmd: $*"
	exit 1
}

require-env() {
	for arg in "$@"; do
		if [ -z "${!arg-}" ]; then
			die "$arg not set"
		fi
	done
}

cse() {
	ssh -t cse cd __workdir__ \; "$@"
}

fetch() {
	ssh -t cse rm -rf __workdir__ \; mkdir -p __workdir__

	cse "$COURSE" fetch "$TASK"

	scp -r cse:'__workdir__/*' .
}

autotest() {
	rsync --delete -avh . cse:__workdir__

	# -t to get colour output
	ssh -t -o LogLevel=QUIET cse cd __workdir__ \; "$COURSE" autotest "$@"
}

give() {
	if echo "$1" | grep -q '\.'; then
		subtask=${1%.*}
		echo "$(basename "$0"): give: deducing task as '$subtask'"
		give "$subtask" "$@"
		return
	fi

	rsync --delete -avh "${@:2}" cse:__workdir__
	yes yes | cse give "cs$COURSE" "${TASK}_$1" "${@:2}"
}

remote() {
	echo "Running on cse: $*"
	rsync --delete -a . cse:__workdir__
	cse "$@"
}

task() {
	# TODO handle insufficient args
	[ -n "${TASK-}" ] && die "task already set"
	[ -e "$1" ] && die "./$1 already exists"

	export TASK=$1

	mkdir "$TASK"
	cd "$TASK"
	fetch

	echo "TASK=$TASK" > .env
}

main() {
	command="${1-}"
	shift || true

	case "$command" in
		fetch|autotest|give)
			require-env COURSE TASK
			"$command" "$@"
			;;
		task)
			require-env COURSE
			"$command" "$@"
			;;
		remote)
			"$command" "$@"
			;;
		""|help)
			helpmsg
			;;
		*)
			echo "$cmd: unknown command '$command'"
			;;
	esac
}

main "$@"
