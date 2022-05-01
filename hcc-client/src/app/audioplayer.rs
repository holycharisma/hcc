use crate::hooks::use_window_scroll;
use crate::htmx::HtmxProcessedComponent;

// use gloo_console::log;

use yew::prelude::*;

#[function_component(AudioPlayer)]
pub fn audioplayer() -> Html {
    let webamp = html! {
     <div id="webamp-player">
      <div id="webamp-main-window">
        <div id="webamp-main-top"></div>
        <div id="webamp-main-info">
          <div id="webamp-main-info-now-playing-left">
            <div id="webamp-main-info-now-playing-indicator">
              <span>
                {"⏹"}
              </span>
            </div>
            <div id="webamp-main-info-now-playing-digits">
              <div id="webamp-main-info-now-playing-minutes">
                <span id="webamp-now-playing-minute-digit-3"></span>
                <span id="webamp-now-playing-minute-digit-2">{"1"}</span>
                <span id="webamp-now-playing-minute-digit-1">{"1"}</span>
              </div>
              <div id="webamp-main-info-now-playing-seconds">
                <span id="webamp-now-playing-second-digit-1">{"1"}</span>
                <span id="webamp-now-playing-second-digit-2">{"1"}</span>
              </div>
            </div>
          </div>
          <div id="webamp-main-info-now-playing-right">
            <div class="marquee"
                 id="webamp-main-info-now-playing-title-marquee">
              <div>
                  <span>{"01. 2022 - cool song.mp3 * * *"}</span>
                  <span>{"01. 2022 - cool song.mp3 * * *"}</span>
                  <span>{"01. 2022 - cool song.mp3 * * *"}</span>
                  <span>{"01. 2022 - cool song.mp3 * * *"}</span>
                  <span>{"01. 2022 - cool song.mp3 * * *"}</span>
                  <span>{"01. 2022 - cool song.mp3 * * *"}</span>
                  <span>{"01. 2022 - cool song.mp3 * * *"}</span>
                  <span>{"01. 2022 - cool song.mp3 * * *"}</span>
              </div>
            </div>
            <div id="webamp-main-info-now-playing-kbps">
              <span>{"192"}</span>
            </div>
            <div id="webamp-main-info-now-playing-khz">
              <span>{"44"}</span>
            </div>
          </div>
          <div id="webamp-main-info-volume">
            // <!-- can animate via margin-left property from 0-62% -->
            <div id="webamp-main-info-volume-position" draggable="true">
            </div>
          </div>
          <div id="webamp-main-info-overlay">
          </div>
        </div>
        <div id="webamp-main-seek">
          <div id="webamp-main-seek-playhead" draggable="true"></div>
        </div>
        <div id="webamp-main-controls">
          <div id="webamp-main-controls-actions">
            <div id="webamp-main-controls-action-back"></div>
            <div id="webamp-main-controls-action-play"></div>
            <div id="webamp-main-controls-action-pause"></div>
            <div id="webamp-main-controls-action-stop"></div>
            <div id="webamp-main-controls-action-forward"></div>
          </div>
          <div id="webamp-main-controls-overlay"></div>
          <a href="https://archive.org/details/winampskins_OS8_AMP_-_Magenta" target="_blank">
          <div id="webamp-main-controls-info"></div>
          </a>
      </div>
      <div id="webamp-playlist-window">
        <div id="webamp-playlist-top">
          <div id="webamp-playlist-top-left"></div>
          <div class="webamp-playlist-spacer"></div>
          <div class="webamp-playlist-spacer"></div>
          <div class="webamp-playlist-spacer"></div>
          <div id="webamp-playlist-top-title"></div>
          <div class="webamp-playlist-spacer"></div>
          <div class="webamp-playlist-spacer"></div>
          <div class="webamp-playlist-spacer"></div>
          <div id="webamp-playlist-top-right"></div>
        </div>
        <div id="webamp-playlist-middle">
          <div id="webamp-playlist-middle-left">
            <div class="repeat-y"></div>
            <div class="repeat-y"></div>
            <div class="repeat-y"></div>
          </div>
          <div id="webamp-playlist-middle-center">
            <div id="webamp-playlist-middle-contents">
              <ol>
                <li class="active selected">
                  <span class="num">{"01"}</span>
                  <span class="title">{"2022 - cool song.mp3"}</span>
                  <span class="duration">{"0:36"}</span>
                </li>
                <li>
                  <span class="num">{"02"}</span>
                  <span class="title">{"2021 - other song - written and recorded by holy charismos.mp3"}</span>
                  <span class="duration">{"1:24"}</span>
                </li>
              </ol>
            </div>
          </div>
          <div id="webamp-playlist-middle-right">
            <div class="repeat-y"></div>
            <div class="repeat-y"></div>
            <div class="repeat-y"></div>
            <div id="webamp-playlist-middle-scroll" draggable="true"></div>
          </div>
        </div>
        <div id="webamp-playlist-bottom">
          <div id="webamp-playlist-bottom-left">
            <div id="webamp-playlist-bottom-overlay">
              <hr />
            </div>
          </div>
          <div id="webamp-playlist-bottom-center"></div>
          <div id="webamp-playlist-bottom-right"></div>
        </div>
      </div>
      </div>
    </div>
      };

    html! {
        webamp
        // <div>{"I am an audioplayer"}</div>
    }
}
