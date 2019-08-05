import React from 'react';

import Link from 'next/link';

const layoutStyle = {
  margin: 20,
  padding: 20,
  border: '1px solid #DDD'
};

export default class ThreadTile extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      ...props.thread
    };
  }

  render() {
    const { thread_id, user_id, title, created_at, updated_at } = this.state;

    return (
      <div style={layoutStyle}>
        <Link href={`/thread?id=${thread_id}`}>
          <a>{title}</a>
        </Link>
      </div>
    )
  }
}
