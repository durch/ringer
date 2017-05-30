import checkSession from './Login';
import './index.css';
require('basscss/css/basscss.css');
require('normalize.css/normalize.css');


const buildUrl = function (props) {
  return `${props.proto}://${props.host}/${props.api_version}`
};

let defaultProps = {
  // proto: "https",
  // host: "ringer.adex.tech",
  proto: "http",
  host: "localhost:5000",
  api_version: "v0",
  esper_url: "https://esper.adex.tech/subscribe/checks12",
  // key: "lala",
  expanded: true,
  stale: true,
  sessionValid: false
}

defaultProps.url = buildUrl(defaultProps);

checkSession(defaultProps);

const evtSource = new EventSource(defaultProps.esper_url);
evtSource.onmessage = function (evt) {
  defaultProps.list = JSON.parse(evt.data.trim());
  checkSession(defaultProps);
};

