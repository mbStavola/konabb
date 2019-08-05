import React from "react";
import { withRouter } from 'next/router';

import PostTile from "../components/postTile";
import ThreadTile from "../components/threadTile";

import * as KonabbApi from '../utils/api'
import Layout from "../components/layout";

const layoutStyle = {
  margin: 20,
  padding: 20,
  border: '1px solid #DDD'
};

class PostCreator extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      text: ''
    };
  }

  handleSubmission() {
    const { onSubmit } = this.props;
    const { text } = this.state;

    onSubmit(text);
    this.setState({ text: '' });
  }

  render() {
    const { text } = this.state;

    return (
      <div style={layoutStyle}>
        Create New Post
        <input type="text" name="text" value={text} onChange={(event) => this.setState({ text: event.target.value })}/>
        <button onClick={() => this.handleSubmission()}>Submit</button>
      </div>
    )
  }
}

class Thread extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      thread: {},
      posts: [],
      page: 0,
      size: 20,
      pages: 0
    };
  }

  async componentDidMount() {
    const { router } = this.props;
    await this.stateFromThreadId(router.query.id);
  }

  async stateFromThreadId(threadId) {
    const threadData = await KonabbApi.getThread(threadId);
    this.setState({ ...threadData });
  }

  async handlePostCreation(text) {
    const { thread } = this.state;

    const post = {
      thread_id: thread.thread_id,
      text: text
    };

    await KonabbApi.createPost(post);
    await this.stateFromThreadId(thread.thread_id);
  }

  render() {
    const { thread, posts } = this.state;

    return (
      <Layout>
        <div>
          <ThreadTile key={thread.thread_id} thread={thread}/>
        </div>
        <div>
          {posts.map(post => <PostTile key={post.post_id} post={post}/>)}
        </div>
        <PostCreator onSubmit={(text) => this.handlePostCreation(text)}/>
      </Layout>
    )
  }
}

export default withRouter(Thread);
