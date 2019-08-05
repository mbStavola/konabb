import React from "react";

import * as KonabbApi from '../utils/api';

const layoutStyle = {
  margin: 20,
  padding: 20,
  border: '1px solid #DDD'
};

export class LoginTile extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      username: '',
      password: ''
    };
  }

  async handleLogin() {
    const { onLogin } = this.props;
    const { username, password } = this.state;

    const userSession = await KonabbApi.loginUser(username, password);

    onLogin(userSession);
    this.setState({ username: '', password: '' });
  }

  render() {
    const { username, session } = this.state;

    return (
      <div style={layoutStyle}>
        Login
        <input type="text" name="username" placeholder="username" value={username}
               onChange={(event) => this.setState({ username: event.target.value })}/>
        <input type="password" name="password" placeholder="password" value={session}
               onChange={(event) => this.setState({ password: event.target.value })}/>
        <button onClick={() => this.handleLogin()}>Login</button>
      </div>
    )
  }
}
