import React from "react";
import { withRouter } from 'next/router';

import ThreadTile from '../components/threadTile'

import * as KonabbApi from '../utils/api'
import Layout from "../components/layout";

const layoutStyle = {
  margin: 20,
  padding: 20,
  border: '1px solid #DDD'
};

class ThreadCreator extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      title: '',
      text: ''
    };
  }

  handleSubmission() {
    const { onSubmit } = this.props;
    const { title, text } = this.state;

    onSubmit(title, text);
    this.setState({ title: '', text: '' });
  }

  render() {
    const { title, text } = this.state;

    return (
      <div style={layoutStyle}>
        Create New Thread
        <input type="text" name="title" value={title}
               onChange={(event) => this.setState({ title: event.target.value })}/>
        <input type="text" name="text" value={text} onChange={(event) => this.setState({ text: event.target.value })}/>
        <button onClick={() => this.handleSubmission()}>Submit</button>
      </div>
    )
  }
}


class Board extends React.Component {
  constructor(props) {
    super(props);

    this.state = {
      board: {},
      threads: [],
      page: 0,
      size: 20,
      pages: 0,
    };
  }

  async componentDidMount() {
    const { router } = this.props;
    const boardData = await KonabbApi.getBoard(router.query.name);
    this.setState({ ...boardData });
  }

  async handleThreadCreation(boardId, title, text) {
    const { router } = this.props;

    const thread = {
      board_id: boardId,
      title,
      text
    };

    const threadId = await KonabbApi.createThread(thread);
    router.push(`/thread?id=${threadId}`);
  }

  render() {
    const { board, threads } = this.state;

    return (
      <Layout>
        <div>
          {threads.map(thread => <ThreadTile key={thread.thread_id} thread={thread}/>)}
        </div>
        <ThreadCreator onSubmit={(title, text) => this.handleThreadCreation(board.board_id, title, text)}/>
      </Layout>
    );
  }
}

export default withRouter(Board);
