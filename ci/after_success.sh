set -euxo pipefail

main() {
    cargo doc

    mkdir ghp-import

    curl -Ls https://github.com/davisp/ghp-import/archive/master.tar.gz | \
        tar --strip-components 1 -C ghp-import -xz

    ./ghp-import/ghp_import.py target/doc

    set +x
    printf "%s" "${GH_DEPLOY_KEY_BASE64}" | base64 -d > "${HOME}/.ssh/github_deploy_key"
    set -x

    chmod 600 "${HOME}/.ssh/github_deploy_key"
    cp ci/sshconfig "${HOME}/.ssh/config"
    git push -f git@github.com:ComputerScienceHouse/csh-bot.git gh-pages
}

if [ "$TRAVIS_BRANCH" == "master" ]; then
    main
fi