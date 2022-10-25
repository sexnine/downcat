# Downcat Linux installer https://github.com/sexnine/downcat
# Install script made with huge help by jethro#1547

echo "Welcome to the downcat installer.  This will download and install the latest version of downcat."
echo ''
echo "Downloading latest release..."
echo ''
# Below if statement checks if required files already exist, so if the script is restarted there are no duplicated files

if [ ! -e "downcat" ] || [ ! -e "x86_64-unknown-linux-gnu.zip" ]; then
	# The below curl command uses the GitHub API to get the download link for the latest release and parses it to wget to install
        curl --silent https://api.github.com/repos/sexnine/downcat/releases/latest | grep 'browser_download_url' | sed -r 's/^[^:]*:(.*)$/\1/' | grep "x86_64-unknown-linux-gnu.zip" | xargs wget --quiet
        echo "Unzipping..."
        unzip -q ./x86_64-unknown-linux-gnu.zip
else
	echo "One or multiple installation files already exist in this directory, continuing without re-downloading..."

fi

chmod +x downcat # Makes program executable, required to run from /usr/bin and also generally a good idea.


echo ''
printf 'Would you like to copy the program to /usr/bin? It requires you to run this script as root (y/n): '
read USRBIN </dev/tty
# The most annoying if statement I've had to write in a while. It was harder to make than it looks.
if [ "$USRBIN" = "y" ]; then
	if [ "$EUID" -ne 0 ]; then
		echo ''
		echo 'You are not running as root. Please restart the script.'
		exit 1
	fi
	echo ''
	echo "Moving now..."
	mv downcat /usr/bin/downcat

fi

echo "Cleaning up zip file..."
rm x86_64-unknown-linux-gnu.zip

echo ''
echo 'Installation done! Run "downcat --help" to start.'