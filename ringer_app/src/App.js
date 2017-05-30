import React from 'react';
import ReactDOM from 'react-dom';
import axios from 'axios';
import EventSource from 'react-eventsource'
import checkSession from './Login.js';
import MainAppBar from './MainAppBar.js'
// material ui theme
import injectTapEventPlugin from 'react-tap-event-plugin';
injectTapEventPlugin();

import CSSTransitionGroup from 'react-transition-group/CSSTransitionGroup'

import './App.css';

import List from './List.js'
import AddCheckForm from './AddCheck.js'

// material ui theme
import getMuiTheme from 'material-ui/styles/getMuiTheme';
import MuiThemeProvider from 'material-ui/styles/MuiThemeProvider';


const renderApp = function (props) {
  if (props.stale) {
    axios.get(`${props.url}/check:list?session_id=${props.session_id}`)
      .then(res => {
        props.list = res.data;
        props.stale = false;
        renderApp(props)
      })
  } else {
    ReactDOM.render(
      <App props={props} />,
      document.getElementById('root')
    )
  }
};

const Transition = function ({ props }) {
  const { component } = props;
  return (
    <CSSTransitionGroup
      transitionName="transition1"
      transitionEnter={false}
      transitionLeaveTimeout={300}
      transitionAppear={true}
      transitionAppearTimeout={500}>
      {component}
    </CSSTransitionGroup>
  )
}

const t = function (component) {
  return (<Transition props={{
    component: component
  }} />)
}


const App = function ({ props }) {
  return (
    <MuiThemeProvider muiTheme={getMuiTheme()}>
      <div>
        <MainAppBar />
        <div className="flex flex-wrap p2">
          <div className="col-7">
            {t(<List props={props} />)}
          </div>
          <div className="px2 col-3">
            {t(<AddCheckForm props={props} />)}
          </div>
        </div>
      </div>
    </MuiThemeProvider>

  )
};

export default renderApp;