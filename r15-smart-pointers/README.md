1. Reference in rust is a pointer.
   - Only allows to access the value at location the
     pointer points to.
2. Smart pointers are types which:
   - Have some metadata.
   - Have some additional capabilities or guarantees.
3. 'String' is a smart pointer.
   - Have 'capacity' as metadata.
   - Guarantees valid UTF-8.
