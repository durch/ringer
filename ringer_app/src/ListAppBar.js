import React from 'react';
import AppBar from 'material-ui/AppBar'
import IconButton from 'material-ui/IconButton';
import RadioButtonChecked from 'material-ui/svg-icons/toggle/radio-button-checked';
import RadioButtonUnchecked from 'material-ui/svg-icons/toggle/radio-button-unchecked';
import Refresh from 'material-ui/svg-icons/navigation/refresh';
import Search from 'material-ui/svg-icons/action/search';
import TextField from 'material-ui/TextField';
import axios from 'axios';

import renderApp from './App.js'

const expandMoreLess = function (props) {
  props.expanded = !props.expanded;
  renderApp(props)
}

const searchFun = function (event, newvalue, props) {
  axios.post(`${props.url}/page:search?session_id=${props.session_id}&value=${newvalue}`).then(res => {
    props.list = res.data;
    props.expanded = true;
    renderApp(props)
  }
  )
}

const AppBarSearch = function ({ props }) {
  return (
    <span className="px4" style={{ color: "#ffffff" }}>
      <Search style={{ color: '#ffffff' }} />
      <TextField name="appbar-search" className="px1" inputStyle={{ color: "#ffffff" }}
        onChange={function (event, newvalue) {
          if (newvalue.length >= 3 || newvalue.length === 0) {
            searchFun(event, newvalue, props)
          }
        }} />
    </span>
  )
};

const ListAppBar = function ({ props }) {
  let { expanded } = props;
  const icon = expanded ? <RadioButtonChecked onClick={function () {
    expandMoreLess(props)
  }} /> : <RadioButtonUnchecked onClick={function () {
    expandMoreLess(props)
  }} />;
  return (
    <AppBar title={
      <span>
        Checks
        {/*<AppBarSearch props={props} />*/}
      </span>
    }
      showMenuIconButton={false}
      iconElementLeft={<IconButton>{icon}</IconButton>}
      iconElementRight={
        <IconButton onClick={function () {
          props.expanded = true;
          renderApp(props)
        }}><Refresh /></IconButton>
      }>
    </AppBar>
  )
}

export default ListAppBar;
