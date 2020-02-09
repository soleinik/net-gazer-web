// automatically generated by the FlatBuffers compiler, do not modify



use std::mem;
use std::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::EndianScalar;

pub enum MessageOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Message<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Message<'a> {
    type Inner = Message<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Message<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Message {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args MessageArgs<'args>) -> flatbuffers::WIPOffset<Message<'bldr>> {
      let mut builder = MessageBuilder::new(_fbb);
      builder.add_seq(args.seq);
      if let Some(x) = args.hops { builder.add_hops(x); }
      if let Some(x) = args.routes { builder.add_routes(x); }
      builder.finish()
    }

    pub const VT_SEQ: flatbuffers::VOffsetT = 4;
    pub const VT_ROUTES: flatbuffers::VOffsetT = 6;
    pub const VT_HOPS: flatbuffers::VOffsetT = 8;

  #[inline]
  pub fn seq(&self) -> u64 {
    self._tab.get::<u64>(Message::VT_SEQ, Some(0)).unwrap()
  }
  #[inline]
  pub fn routes(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Route<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<Route<'a>>>>>(Message::VT_ROUTES, None)
  }
  #[inline]
  pub fn hops(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Hop<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<Hop<'a>>>>>(Message::VT_HOPS, None)
  }
}

pub struct MessageArgs<'a> {
    pub seq: u64,
    pub routes: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<Route<'a >>>>>,
    pub hops: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<Hop<'a >>>>>,
}
impl<'a> Default for MessageArgs<'a> {
    #[inline]
    fn default() -> Self {
        MessageArgs {
            seq: 0,
            routes: None,
            hops: None,
        }
    }
}
pub struct MessageBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> MessageBuilder<'a, 'b> {
  #[inline]
  pub fn add_seq(&mut self, seq: u64) {
    self.fbb_.push_slot::<u64>(Message::VT_SEQ, seq, 0);
  }
  #[inline]
  pub fn add_routes(&mut self, routes: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<Route<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Message::VT_ROUTES, routes);
  }
  #[inline]
  pub fn add_hops(&mut self, hops: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<Hop<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Message::VT_HOPS, hops);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> MessageBuilder<'a, 'b> {
    let start = _fbb.start_table();
    MessageBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Message<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum RouteOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Route<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Route<'a> {
    type Inner = Route<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Route<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Route {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args RouteArgs) -> flatbuffers::WIPOffset<Route<'bldr>> {
      let mut builder = RouteBuilder::new(_fbb);
      builder.add_dst(args.dst);
      builder.add_src(args.src);
      builder.add_route_id(args.route_id);
      builder.finish()
    }

    pub const VT_ROUTE_ID: flatbuffers::VOffsetT = 4;
    pub const VT_SRC: flatbuffers::VOffsetT = 6;
    pub const VT_DST: flatbuffers::VOffsetT = 8;

  #[inline]
  pub fn route_id(&self) -> u16 {
    self._tab.get::<u16>(Route::VT_ROUTE_ID, Some(0)).unwrap()
  }
  #[inline]
  pub fn src(&self) -> u32 {
    self._tab.get::<u32>(Route::VT_SRC, Some(0)).unwrap()
  }
  #[inline]
  pub fn dst(&self) -> u32 {
    self._tab.get::<u32>(Route::VT_DST, Some(0)).unwrap()
  }
}

pub struct RouteArgs {
    pub route_id: u16,
    pub src: u32,
    pub dst: u32,
}
impl<'a> Default for RouteArgs {
    #[inline]
    fn default() -> Self {
        RouteArgs {
            route_id: 0,
            src: 0,
            dst: 0,
        }
    }
}
pub struct RouteBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> RouteBuilder<'a, 'b> {
  #[inline]
  pub fn add_route_id(&mut self, route_id: u16) {
    self.fbb_.push_slot::<u16>(Route::VT_ROUTE_ID, route_id, 0);
  }
  #[inline]
  pub fn add_src(&mut self, src: u32) {
    self.fbb_.push_slot::<u32>(Route::VT_SRC, src, 0);
  }
  #[inline]
  pub fn add_dst(&mut self, dst: u32) {
    self.fbb_.push_slot::<u32>(Route::VT_DST, dst, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> RouteBuilder<'a, 'b> {
    let start = _fbb.start_table();
    RouteBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Route<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum HopOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Hop<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Hop<'a> {
    type Inner = Hop<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Hop<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Hop {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args HopArgs) -> flatbuffers::WIPOffset<Hop<'bldr>> {
      let mut builder = HopBuilder::new(_fbb);
      builder.add_hop(args.hop);
      builder.add_rtt(args.rtt);
      builder.add_route_id(args.route_id);
      builder.add_ttl(args.ttl);
      builder.finish()
    }

    pub const VT_ROUTE_ID: flatbuffers::VOffsetT = 4;
    pub const VT_HOP: flatbuffers::VOffsetT = 6;
    pub const VT_TTL: flatbuffers::VOffsetT = 8;
    pub const VT_RTT: flatbuffers::VOffsetT = 10;

  #[inline]
  pub fn route_id(&self) -> u16 {
    self._tab.get::<u16>(Hop::VT_ROUTE_ID, Some(0)).unwrap()
  }
  #[inline]
  pub fn hop(&self) -> u32 {
    self._tab.get::<u32>(Hop::VT_HOP, Some(0)).unwrap()
  }
  #[inline]
  pub fn ttl(&self) -> u8 {
    self._tab.get::<u8>(Hop::VT_TTL, Some(0)).unwrap()
  }
  #[inline]
  pub fn rtt(&self) -> u16 {
    self._tab.get::<u16>(Hop::VT_RTT, Some(0)).unwrap()
  }
}

pub struct HopArgs {
    pub route_id: u16,
    pub hop: u32,
    pub ttl: u8,
    pub rtt: u16,
}
impl<'a> Default for HopArgs {
    #[inline]
    fn default() -> Self {
        HopArgs {
            route_id: 0,
            hop: 0,
            ttl: 0,
            rtt: 0,
        }
    }
}
pub struct HopBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> HopBuilder<'a, 'b> {
  #[inline]
  pub fn add_route_id(&mut self, route_id: u16) {
    self.fbb_.push_slot::<u16>(Hop::VT_ROUTE_ID, route_id, 0);
  }
  #[inline]
  pub fn add_hop(&mut self, hop: u32) {
    self.fbb_.push_slot::<u32>(Hop::VT_HOP, hop, 0);
  }
  #[inline]
  pub fn add_ttl(&mut self, ttl: u8) {
    self.fbb_.push_slot::<u8>(Hop::VT_TTL, ttl, 0);
  }
  #[inline]
  pub fn add_rtt(&mut self, rtt: u16) {
    self.fbb_.push_slot::<u16>(Hop::VT_RTT, rtt, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> HopBuilder<'a, 'b> {
    let start = _fbb.start_table();
    HopBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Hop<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

#[inline]
pub fn get_root_as_message<'a>(buf: &'a [u8]) -> Message<'a> {
  flatbuffers::get_root::<Message<'a>>(buf)
}

#[inline]
pub fn get_size_prefixed_root_as_message<'a>(buf: &'a [u8]) -> Message<'a> {
  flatbuffers::get_size_prefixed_root::<Message<'a>>(buf)
}

#[inline]
pub fn finish_message_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<Message<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_message_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<Message<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
