import React from 'react';
import axios from 'axios';
import renderApp from './App.js';
import RaisedButton from 'material-ui/RaisedButton';
import FloatingActionButton from 'material-ui/FloatingActionButton';
import Paper from 'material-ui/Paper';
import DeleteForever from 'material-ui/svg-icons/action/delete-forever';
import {
  Table,
  TableBody,
  TableHeader,
  TableHeaderColumn,
  TableRow,
  TableRowColumn,
} from 'material-ui/Table';

export const runCheck = function (id, props) {
  axios.get(`${props.url}/check:run?session_id=${props.session_id}&id=${id}`).then(setTimeout(function () {
    renderApp(props)
  }, 1000))
}

export const deleteCheck = function (id, props) {
  axios.delete(`${props.url}/check:delete?session_id=${props.session_id}&id=${id}`).then(setTimeout(function () {
    renderApp(props)
  }, 1000))
}


const ListTable = function ({ props }) {
  const { list } = props;
  return (
    <Paper zDepth={2}>
      <Table>
        <TableHeader displaySelectAll={false}>
          <TableRow>
            <TableHeaderColumn><span className="h4">URL</span></TableHeaderColumn>
            <TableHeaderColumn className="col-3"><span className="h4">Last run</span></TableHeaderColumn>
            <TableHeaderColumn className="col-2"><span className="h4">Status</span></TableHeaderColumn>
            <TableHeaderColumn className="col-2"><span className="h4">Delete</span></TableHeaderColumn>
          </TableRow>
        </TableHeader>
        <TableBody displayRowCheckbox={false}>
          {list.map(item =>
            <TableRow key={item.id}>
              <TableRowColumn><a className="h4" target="_blank" href={item.url}>{item.url}</a></TableRowColumn>
              <TableRowColumn className="col-3">{item.humanized_end}</TableRowColumn>
              <TableRowColumn className="col-2"><RaisedButton label={item.http_status} secondary={item.http_status >= 400} primary={item.http_status < 400}
                onClick={function () {
                  runCheck(item.id, props)
                }} /></TableRowColumn>
              <TableRowColumn className="col-2"><FloatingActionButton secondary={true} mini={true} zDepth={1}
                onClick={function () {
                  deleteCheck(item.id, props)
                }} ><DeleteForever /></FloatingActionButton></TableRowColumn>
            </TableRow>
          )}
        </TableBody>
      </Table>
    </Paper>
  )
};

export default ListTable;


