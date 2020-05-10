cd "$(dirname "$0")"

# Get the name of the attached atom controller from the o2m output,
# downcased with spaces substituted with underscores
CONTROLLER=$(o2m -l | grep -i atom | awk -F ': ' '{print tolower($2)}' | sed 's/ /_/g')

m2o > /dev/null &
o2m > /dev/null &

sleep 3

./sequencer/target/release/sequencer --controller $CONTROLLER > /dev/null &
sclang ./sampler/sampler.scd > /dev/null &
