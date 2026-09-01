$ devinit add redis ->
get request to web service
{ "redis"{ "version": latest } }
latest is if didnt input version;

$ devinit add redis -v 1.0 
{ "redis" { "version": "1.0" } }

if web service recognise correctly it will
return necessary data with struct Recipe


after choosing version user should apply
and enter Yes or No

after that cli send get request


at the end devinit will support github like
function
it will import all requirements from repo like space.

example:
$devinit get https://devinit.com/gorkyi-chocolate/rust-backend-cli

and export all requirements 
