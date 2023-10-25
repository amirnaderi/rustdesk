set sh1 to "launchctl unload -w /Library/LaunchDaemons/com.carriez.DsHelpDesk_service.plist;"
set sh2 to "/bin/rm /Library/LaunchDaemons/com.carriez.DsHelpDesk_service.plist;"
set sh3 to "/bin/rm /Library/LaunchAgents/com.carriez.DsHelpDesk_server.plist;"

set sh to sh1 & sh2 & sh3
do shell script sh with prompt "DsHelpDesk want to unload daemon" with administrator privileges