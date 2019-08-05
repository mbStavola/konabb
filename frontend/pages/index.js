import React from 'react';

import Layout from '../components/layout.js';

import * as KonabbApi from '../utils/api'
import BoardTile from "../components/boardTile";

export default class Index extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      boards: []
    }
  }

  async componentDidMount() {
    const boards = await KonabbApi.listBoards();
    this.setState({ boards });
  }

  render() {
    const { boards } = this.state;

    return (
      <Layout>
        {boards.map(board => (
          <BoardTile key={board.board_id} board={board}/>
        ))}
      </Layout>
    )
  }
}
