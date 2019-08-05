import React from 'react';

import Link from 'next/link';

const layoutStyle = {
  margin: 20,
  padding: 20,
  border: '1px solid #DDD'
};

export default class BoardTile extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      ...props.board
    };
  }

  render() {
    const { name, short_name, board_type, description } = this.state;

    return (
      <div style={layoutStyle}>
        <Link href={`/board?name=${short_name}`}>
          <a>{name}</a>
        </Link>
      </div>
    )
  }
}
