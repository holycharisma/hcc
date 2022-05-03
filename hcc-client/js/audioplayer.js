import { Howl, Howler } from 'howler';

// some glue to make howl accessible as a playlist instead of a single soundboard
// https://github.com/goldfire/howler.js/blob/master/examples/player/player.js

function createAudioNode(node) {

  let { title, duration, khz, kbps, url } = node;

  node["seek"] = 0;

  node["howl"] = new Howl({
    src: url,
    preload: false,
    onplay: function() {
      // console.log("howl play", this.url);
      this["duration"] = Math.round(this.howl.duration());
      requestAnimationFrame(trackPlayProgress)
    }.bind(node),
    onload: function() {
      // console.log("howl load", this.url);
    }.bind(node),
    onend: function() {
      // console.log("howl end", this.url);
      forward();
    }.bind(node),
    onpause: function() {
      // console.log("howl pause", this.url);
    }.bind(node),
    onstop: function() {
      // console.log("howl stop", this.url);
    }.bind(node),
    onseek: function() {
      // console.log("howl seek", this.url);
    }.bind(node)
  });

  return node;

}

function trackPlayProgress() {
  let node = getCurrentNode();
  let sound = node && node.howl;
  if (sound && sound.playing()) {
    let position = sound.seek() || 0;
    let position_seconds = Math.round(position);
    if (position_seconds !== node.seek) {
      node.seek = position_seconds;
      fireMessage("seek");
    }
    requestAnimationFrame(trackPlayProgress);
  }
}

let sub_key = "subscription";
const UNINITIALIZED = 255;  // uninitialized magic value...
const AudioPlayer$ = {
  [sub_key]: undefined,
  playlist: [],
  current: UNINITIALIZED,
  selected: UNINITIALIZED,
  state: "stop"
};

function is_initialized() {
  return AudioPlayer$.current !== UNINITIALIZED && AudioPlayer$.selected !== UNINITIALIZED;
}

function select(idx) {

  if (AudioPlayer$.current === UNINITIALIZED) {
    AudioPlayer$.current = idx;
  }

  let item = AudioPlayer$.playlist[idx];

  if (item) {
    AudioPlayer$.selected = idx;
    fireMessage("select");
  }

}


function getCurrentNode() {
  let node = AudioPlayer$.playlist[AudioPlayer$.current];
  if (node && node.howl) {
    if (node.howl.state() === "unloaded") {
      node.howl.load();
    }
  }
  return node;
}

function setSubscription(elem) {
  AudioPlayer$[sub_key] = elem;
}

function getSubscription() {
  return AudioPlayer$[sub_key];
}

function pushAudioNode(nodeMeta) {
  let node = AudioPlayer$.playlist.find(f => f.url === nodeMeta.url);
  if (!node) {
    node = createAudioNode(nodeMeta);
    node.num = AudioPlayer$.playlist.push(node);
    let idx = node.num - 1;
    if (!is_initialized()) {
      select(idx);
    }
  }
  return node;
}

function audioMessage(name) {
  return {
    msg: name,
    player: {
      selected: AudioPlayer$.selected,
      current: AudioPlayer$.current,
      state: AudioPlayer$.state,
      tracks: AudioPlayer$.playlist
    }
  }
}

function fireEvent(data) {
  let elem = getSubscription();

  if (elem) {
    const event = new CustomEvent("audioplayer", {
      detail: data
    });

    elem.dispatchEvent(event);

  }

}

function fireMessage(message) {
  fireEvent(audioMessage(message));
}

function subscribe(el) {
  setSubscription(el);
  fireMessage("subscribe");
}

function stopInternal(sound) {
  if (sound && sound.howl) {
    sound.howl.stop();
    sound.seek = 0;
  }
}

function stop() {
  let sound = getCurrentNode();
  if (sound && sound.howl) {
    stopInternal(sound);
    AudioPlayer$.state = "stop";
    fireMessage("stop");
  }
}

function play() {
  if (AudioPlayer$.current !== AudioPlayer$.selected) {
    stop();
    AudioPlayer$.current = AudioPlayer$.selected;
  }
  let sound = getCurrentNode();
  if (sound && sound.howl && !sound.howl.playing()) {
    sound.howl.play();
    AudioPlayer$.state = "play";
    fireMessage("play");
  }
}

function pause() {
  let sound = getCurrentNode();
  if (sound && sound.howl && sound.howl.playing()) {
    sound.howl.pause();
    AudioPlayer$.state = "pause";
    fireMessage("pause");
  }
}

function handleSkip() {
  if (AudioPlayer$.state === "stop") {
    stop();
  } else if (AudioPlayer$.state === "pause") {
    pause();
  } else {
    play();
  }
}

function seekToStart() {

  let sound = getCurrentNode();
  if (sound && sound.howl && sound.howl.playing()) {
    sound.howl.seek(0);
    sound.seek = 0;
    fireMessage("seek");
  }

}

function forward() {
  // stop current, if next, set selected to next and play
  let sound = getCurrentNode();
  if (sound && sound.howl && sound.howl.playing()) {
    stopInternal(sound);
  }

  if (AudioPlayer$.playlist.length > AudioPlayer$.current + 1) {
    AudioPlayer$.current += 1;
    AudioPlayer$.selected = AudioPlayer$.current;
    seekToStart();
    fireMessage("forward");
    handleSkip();
  }

}

function back() {
  // stop current, if prev, set selected to prev and play
  let sound = getCurrentNode();
  if (sound && sound.howl) {
    sound.howl.stop();
  }

  if (AudioPlayer$.current - 1 >= 0) {
    AudioPlayer$.current -= 1;
    AudioPlayer$.selected = AudioPlayer$.current;
    seekToStart();
    fireMessage("back");
    handleSkip();
  }

}


function push(title, url, duration, khz, kbps) {
  pushAudioNode({
    title: title,
    duration: duration,
    khz: khz,
    kbps: kbps,
    url: url
  });
  
}

window.audioplayer = {
  subscribe: subscribe,
  select: select,
  stop: stop,
  play: play,
  pause: pause,
  forward: forward,
  back: back,
  push: push
};
