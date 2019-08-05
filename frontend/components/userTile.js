import React from "react";

import * as KonabbApi from '../utils/api';

const layoutStyle = {
  margin: 20,
  padding: 20,
  border: '1px solid #DDD'
};

export default class UserTile extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      user: {}
    }
  }

  async componentDidMount() {
    const { userId, onFetch } = this.props;
    const user = await KonabbApi.getUser(userId);
    this.setState({ user });
    onFetch(user);
  }

  render() {
    const { user } = this.state;
    return (
      <div style={layoutStyle}>
        {user.username}
      </div>
    );
  }
}
