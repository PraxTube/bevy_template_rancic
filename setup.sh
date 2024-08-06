binary=""
itch=""

read -p "Name of game binary: " binary
read -p "Name of itch page: " itch
binary="${binary//\//\\\/}"
itch="${itch//\//\\\/}"

rm -rf .git
git init

rm -rf target
rm Cargo.lock

sed -i "s/bevy_template_rancic/${binary}/g" push_to_itch.sh
sed -i "s/PraxTube\/bevy-template-rancic/${itch}/g" push_to_itch.sh

sed -i "s/bevy_template_rancic/${binary}/g" .github/workflows/release.yml
sed -i "s/PraxTube\/bevy-template-rancic/${itch}/g" .github/workflows/release.yml

sed -i "s/bevy_template_rancic/${binary}/g" Cargo.toml

rm setup.sh
