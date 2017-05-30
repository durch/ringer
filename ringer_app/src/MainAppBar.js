import React from 'react';
import AppBar from 'material-ui/AppBar'
import Alarm from 'material-ui/svg-icons/action/alarm';
import IconButton from 'material-ui/IconButton';
import FontIcon from 'material-ui/FontIcon';
import DonateDialog from './DonateDialog.js';

const MainAppBar = function ({ props }) {
    return (
        <AppBar title={
            <span>
                Ringer
            </span>
        }
            showMenuIconButton={true}
            iconElementLeft={<IconButton iconStyle={{ color: "#ffffff" }} onTouchTap={function () { window.open('https://github.com/durch/ringer', '_blank') }}><FontIcon className="fa fa-github" /></IconButton>}
            iconElementRight={<DonateDialog />}
        />
    )
}

export default MainAppBar;
