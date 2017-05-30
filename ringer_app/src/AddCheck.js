import React from 'react';
import checkSession from './Login';
import TextField from 'material-ui/TextField';
import Paper from 'material-ui/Paper';
import IconButton from 'material-ui/IconButton';
import AppBar from 'material-ui/AppBar'
import axios from 'axios';
import AddCircleOutline from 'material-ui/svg-icons/content/add-circle-outline';

export const addCheck = function (url, rate, props) {
    axios.put(`${props.url}/check:add?session_id=${props.session_id}`, {
        "url": url,
        "rate": rate
    }).then(setTimeout(function () {
        checkSession(props)
    }, 2000))
}

const AddCheckForm = function ({ props }) {
    return (
        <span>
            <Paper zDepth={2}>
                <AppBar
                    showMenuIconButton={false}
                    title="Add check"
                    iconElementRight={<IconButton onClick={function () {
                        const url = document.getElementById("addcheck-url").value;
                        const rate = document.getElementById("addcheck-rate").value;
                        addCheck(url, rate, props)
                        document.getElementById("addcheck-url").value = "";
                        document.getElementById("addcheck-rate").value = "";
                    }} ><AddCircleOutline /></IconButton>}
                />
                <TextField id="addcheck-url" floatingLabelText="URL" className="px2" style={{ display: "block" }} />
                <TextField id="addcheck-rate" type="number" floatingLabelText="Rate, seconds" className="px2" />
            </Paper>
        </span>
    )
};

export default AddCheckForm