﻿//HintName: Test.NestingNamespaces.AndClasses.InsertData2.cs
// <auto-generated />
#nullable enable

namespace Test.NestingNamespaces
{
    partial class AndClasses
    {
        public static void VolatileNonatomicScheduleImmediateInsertData2(PublicTable data)
        {
            using var stream = new MemoryStream();
            using var writer = new BinaryWriter(stream);
            new PublicTable.BSATN().Write(writer, data);
            SpacetimeDB.Internal.IReducer.VolatileNonatomicScheduleImmediate(
                "test_custom_name_and_reducer_ctx",
                stream
            );
        }
    } // AndClasses
} // namespace
