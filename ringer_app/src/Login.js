import React from 'react';
import ReactDOM from 'react-dom';
import axios from 'axios';
import Paper from 'material-ui/Paper';
import MainAppBar from './MainAppBar.js';

// import Recaptcha from 'react-recaptcha';

import CSSTransitionGroup from 'react-transition-group/CSSTransitionGroup'

import getMuiTheme from 'material-ui/styles/getMuiTheme';
import MuiThemeProvider from 'material-ui/styles/MuiThemeProvider';
// import darkBaseTheme from 'material-ui/styles/baseThemes/darkBaseTheme';

import RaisedButton from 'material-ui/RaisedButton';
import TextField from 'material-ui/TextField';

import renderApp from './App.js'

const renderLogin = function (props) {
  ReactDOM.render(
    <LoginForm props={props} />,
    document.getElementById('root')
  )
};


const isSessionValid = function (props) {
  // Plug this whole with CORS
  axios.get(`${props.url}/session:validate?session_id=${props.session_id}`)
    .then(res => {
      if (res.data.valid) {
        props.sessionValid = res.data.valid;
        renderApp(props);
      } else {
        renderLogin(props)
      }
    }).catch(err => {
      renderLogin(props)
    })
}


const checkSession = function (props) {
  if (typeof props.session_id !== undefined) {
    isSessionValid(props)
  } else {
    renderLogin(props)
  }
}

const credentials = function (endpoint, props, cb) {
  const email = document.getElementById("email").value;
  const pass = document.getElementById("pass").value;
  axios.post(`${props.url}/user:${endpoint}`, {
    "email": email,
    "pass": pass
  }).then(res => {
    if (endpoint === 'login') {
      props.session_id = res.data;
      checkSession(props)
    } else if (endpoint == 'register') {

    } else {
      renderLogin(props)
    }
  })
}

const login = (props) => credentials("login", props);
const register = (props) => credentials("register", props);


// const getCookie = function (cname) {
//   var name = `${cname}=`;
//   var ca = document.cookie.split(';');
//   for (var i = 0; i < ca.length; i++) {
//     var c = ca[i];
//     while (c.charAt(0) === ' ') {
//       c = c.substring(1);
//     }
//     if (c.indexOf(name) === 0) {
//       return c.substring(name.length, c.length);
//     }
//   }
//   return null;
// }

// const setCookie = function (cname, cvalue, exdays) {
//   var d = new Date();
//   d.setTime(d.getTime() + (exdays * 24 * 60 * 60 * 1000));
//   var expires = `expires=+${d.toUTCString()}`;
//   document.cookie = `${cname}=${cvalue};${expires};path=/`;
// }

// const setSessionCookie = function (session_id) {
//   setCookie("sessionId", session_id, 1)
// }

var verifyCallback = function (response) {
  console.log(response);
};

const Registered = function ({ props }) {

};

const LoginForm = function ({ props }) {
  return (
    <MuiThemeProvider muiTheme={getMuiTheme()}>
      <div>
        <MainAppBar />
        <div className="p2 col-3">
          <Paper zDepth={2}>
            <div className="p2">
              <CSSTransitionGroup
                transitionName="transition1"
                transitionEnter={false}
                transitionLeaveTimeout={300}
                transitionAppear={true}
                transitionAppearTimeout={500}>
                <TextField id="email" floatingLabelText="EMAIL" /><br />
                <TextField id="pass" floatingLabelText="PASSWORD" /><br />
                <RaisedButton label="Login" primary={true} onClick={function () {
                  login(props)
                }} />
                <span className="px1"></span>
                <RaisedButton label="Register" secondary={true} onClick={function () {
                  register(props)
                }} />

                {/*<br />
          <br />
          <Recaptcha
            sitekey="6LfKwSIUAAAAAPl6YXHzfiylRfg0cpkWoGi5rtSN"
            verifyCallback={verifyCallback}
          />*/}

              </CSSTransitionGroup>
            </div>
          </Paper>
        </div>
      </div>
    </MuiThemeProvider>
  )
};

export default checkSession;