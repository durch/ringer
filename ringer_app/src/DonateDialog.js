import React from 'react';
import Dialog from 'material-ui/Dialog';
import FlatButton from 'material-ui/FlatButton';
import IconButton from 'material-ui/IconButton';
import FontIcon from 'material-ui/FontIcon';


export default class DonateDialog extends React.Component {
    state = {
        open: false,
    };

    handleOpen = () => {
        this.setState({ open: true });
    };

    handleClose = () => {
        this.setState({ open: false });
    };

    render() {
        const actions = [
            <FlatButton
                label="Ok"
                primary={true}
                onTouchTap={this.handleClose}
            />,

        ];

        return (
            <div>
                <IconButton iconStyle={{color: "#ffffff"}} onTouchTap={this.handleOpen}><FontIcon className="fa fa-btc"/></IconButton>
                <Dialog
                    title="3AsKzB3DYWY4s8g5xbksDzf2yNh5njqaD8"
                    actions={actions}
                    modal={false}
                    open={this.state.open}
                    onRequestClose={this.handleClose}
                >
                We love Bitcoin, and love keeps us running :)
        </Dialog>
            </div>
        );
    }
}