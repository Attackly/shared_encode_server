ffmpeg -loglevel error -i input.mkv -map 0:v:0 -c copy -f data - | xxhsum -H3

ffmpeg -i distorted.mkv -i reference.mkv -lavfi libvmaf=log_fmt=json:log_path=vmaf_result.json -f null -


usage: 
shared_encode -e <encoder> -q <quality in numbers> -p <preset> (--no-send)
shared_encode -s -i <Input>

this would be for av1 
