var searchIndex = JSON.parse('{\
"oak_io":{"doc":"Shared data structures and functionality for inter-node …","t":[8,8,13,13,13,13,13,13,13,13,13,13,6,3,13,3,4,4,13,13,13,13,3,3,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,12,10,11,11,11,11,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,0,12,12,14,12,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,8,24,3,3,3,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,5,11,11,11,11,10,11,11,11,11,11,11,12,12,12,12,5,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["Decodable","Encodable","ErrBadHandle","ErrBufferTooSmall","ErrChannelClosed","ErrChannelEmpty","ErrHandleSpaceTooSmall","ErrInternal","ErrInvalidArgs","ErrOutOfRange","ErrPermissionDenied","ErrTerminated","Handle","InitWrapper","IoError","Message","OakError","OakStatus","OakStatus","Ok","ProtobufDecodeError","ProtobufEncodeError","Receiver","Sender","Unspecified","as_proto_handle","as_proto_handle","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","bytes","clear","clear","clone","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","clone_into","cmp","command_receiver","decode","decode","default","default","default","encode","encode","encoded_len","encoded_len","eq","eq","eq","eq","eq","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fold","fold","from","from","from","from","from","from","from","from","from","from","from","from_i32","handle","handle","handle","handle_visit_blanket_impl","handles","hash","init","into","into","into","into","into","into","into_inner","is_valid","ne","ne","ne","ne","new","new","partial_cmp","to_owned","to_owned","to_owned","to_owned","to_owned","to_string","to_string","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","0","0","0","0","HandleVisit","HandleVisit","ReadHandle","Receiver","Sender","WriteHandle","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clear","clear","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","default","default","encoded_len","encoded_len","eq","eq","eq","eq","extract_handles","fmt","fmt","fmt","fmt","fold","from","from","from","from","from","from","handle","handle","id","id","inject_handles","into","into","into","into","ne","ne","ne","ne","to_owned","to_owned","to_owned","to_owned","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id"],"q":["oak_io","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","oak_io::OakError","","","","oak_io::handle","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["A trait for objects that can be decoded from bytes + …","A trait for objects that can be encoded as bytes + handles.","Invalid handle provided.","Provided buffer was too small for operation (an output …","Channel has been closed.","Channel has no messages available to read.","Provided handle space was too small for operation (an …","Internal error.","Arguments invalid.","Argument out of valid range.","The node does not have sufficient permissions to perform …","Node terminated.","Handle used to identify read or write channel halves.","A wrapper struct that holds an init message, plus the …","","A simple holder for bytes + handles, using internally …","Generic Oak error.","Status values exchanged as i32 values across the Node Wasm …","","Success.","","","Wrapper for a handle to the read half of a channel, …","Wrapper for a handle to the send half of a channel, …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Converts an <code>i32</code> to a <code>OakStatus</code>, or <code>None</code> if <code>value</code> is not a …","","","","","","","","","","","","","","","Returns <code>true</code> if <code>value</code> is a variant of <code>OakStatus</code>.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Visit all handles present in a type.","Automatically derives the <code>HandleVisit</code> trait for structs …","Wrapper for a handle to the read half of a channel.","Receiver handle for an Oak channel. This type is sent over …","Sender handle for an Oak channel. This type is sent over …","Wrapper for a handle to the send half of a channel.","","","","","","","","","","","","","","","","","","","","","","","","","","","Return all handles in <code>T</code>.","","","","","Invokes the provided closure on every handle contained in …","","","","","","","","","","","Inject handles into a message.","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[0,0,1,1,1,1,1,1,1,1,1,1,0,0,2,0,0,0,2,1,2,2,0,0,1,3,4,1,2,3,4,5,6,1,2,3,4,5,6,5,3,4,1,3,4,5,6,1,3,4,5,6,1,6,7,6,1,3,4,8,6,3,4,1,3,4,5,6,1,1,2,2,3,4,5,6,3,4,1,2,2,2,2,2,3,3,4,5,6,1,0,3,4,0,5,1,6,1,2,3,4,5,6,3,1,3,4,5,6,3,4,1,1,3,4,5,6,1,2,1,2,3,4,5,6,1,2,3,4,5,6,1,2,3,4,5,6,9,10,11,12,0,0,0,0,0,0,13,14,15,16,13,14,15,16,13,14,13,14,15,16,13,14,15,16,13,14,13,14,13,14,15,16,0,13,14,15,16,17,13,14,15,15,16,16,15,16,13,14,0,13,14,15,16,13,14,15,16,13,14,15,16,13,14,15,16,13,14,15,16,13,14,15,16],"f":[null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[],["receiver",3]],[[],["sender",3]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],null,[[]],[[]],[[],["oakstatus",4]],[[],["receiver",3]],[[],["sender",3]],[[],["message",3]],[[],["initwrapper",3]],[[]],[[]],[[]],[[]],[[]],[[["oakstatus",4]],["ordering",4]],null,[[["message",3]],["result",4,[["oakerror",4]]]],[[["message",3]],["result",4,[["oakerror",4]]]],[[],["oakstatus",4]],[[],["receiver",3]],[[],["sender",3]],[[],["result",4,[["message",3],["oakerror",4]]]],[[],["result",4,[["message",3],["oakerror",4]]]],[[],["usize",15]],[[],["usize",15]],[[["oakstatus",4]],["bool",15]],[[["receiver",3]],["bool",15]],[[["sender",3]],["bool",15]],[[["message",3]],["bool",15]],[[["initwrapper",3]],["bool",15]],[[["formatter",3]],["result",4,[["error",3]]]],[[["formatter",3]],["result",4,[["error",3]]]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",4,[["error",3]]]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[["oakstatus",4]]],[[["error",3]]],[[["encodeerror",3]]],[[["decodeerror",3]]],[[]],[[["readhandle",3]]],[[]],[[]],[[]],[[]],[[["i32",15]],["option",4,[["oakstatus",4]]]],null,null,null,null,null,[[]],null,[[]],[[]],[[]],[[]],[[]],[[]],[[],["readhandle",3]],[[["i32",15]],["bool",15]],[[["receiver",3]],["bool",15]],[[["sender",3]],["bool",15]],[[["message",3]],["bool",15]],[[["initwrapper",3]],["bool",15]],[[["readhandle",3]]],[[["writehandle",3]]],[[["oakstatus",4]],["option",4,[["ordering",4]]]],[[]],[[]],[[]],[[]],[[]],[[],["string",3]],[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],null,null,null,null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["sender",3]],[[],["receiver",3]],[[],["readhandle",3]],[[],["writehandle",3]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["usize",15]],[[],["usize",15]],[[["sender",3]],["bool",15]],[[["receiver",3]],["bool",15]],[[["readhandle",3]],["bool",15]],[[["writehandle",3]],["bool",15]],[[],["vec",3,[["handle",6]]]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",4,[["error",3]]]],[[["formatter",3]],["result",4,[["error",3]]]],[[]],[[]],[[]],[[["handle",6]]],[[]],[[]],[[["handle",6]]],null,null,null,null,[[],["result",4,[["oakerror",4]]]],[[]],[[]],[[]],[[]],[[["sender",3]],["bool",15]],[[["receiver",3]],["bool",15]],[[["readhandle",3]],["bool",15]],[[["writehandle",3]],["bool",15]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]]],"p":[[4,"OakStatus"],[4,"OakError"],[3,"Receiver"],[3,"Sender"],[3,"Message"],[3,"InitWrapper"],[8,"Decodable"],[8,"Encodable"],[13,"ProtobufDecodeError"],[13,"ProtobufEncodeError"],[13,"OakStatus"],[13,"IoError"],[3,"Sender"],[3,"Receiver"],[3,"ReadHandle"],[3,"WriteHandle"],[8,"HandleVisit"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};