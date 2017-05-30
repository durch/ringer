import React from 'react';
import CSSTransitionGroup from 'react-transition-group/CSSTransitionGroup'

import ListAppBar from './ListAppBar.js';
import ListTable from './ListTable.js'

const List = function ({ props }) {
  const { expanded } = props;
  const listTable = expanded ? <ListTable key="listtable" props={props} /> : null;
  return (
    <span>
      <ListAppBar props={props} />
      <CSSTransitionGroup
        transitionName="transition1"
        transitionEnterTimeout={500}
        transitionLeaveTimeout={300}>
        {listTable}
      </CSSTransitionGroup>
    </span>
  )
}

export default List;
