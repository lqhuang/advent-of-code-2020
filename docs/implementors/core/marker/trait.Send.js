(function() {var implementors = {};
implementors["day02"] = [{"text":"impl Send for OccurrsPolicy","synthetic":true,"types":[]},{"text":"impl Send for PosMatchPolicy","synthetic":true,"types":[]}];
implementors["day03"] = [{"text":"impl Send for Space","synthetic":true,"types":[]},{"text":"impl Send for Map","synthetic":true,"types":[]},{"text":"impl Send for Slope","synthetic":true,"types":[]}];
implementors["day04"] = [{"text":"impl Send for MUST_FIELDS","synthetic":true,"types":[]},{"text":"impl Send for HAIR_COLORS","synthetic":true,"types":[]},{"text":"impl Send for HeightUnit","synthetic":true,"types":[]},{"text":"impl Send for Height","synthetic":true,"types":[]},{"text":"impl Send for Passport","synthetic":true,"types":[]}];
implementors["day05"] = [{"text":"impl Send for BoardingPass","synthetic":true,"types":[]}];
implementors["day11"] = [{"text":"impl Send for Seat","synthetic":true,"types":[]},{"text":"impl Send for SeatGrid","synthetic":true,"types":[]},{"text":"impl Send for FlapType","synthetic":true,"types":[]}];
implementors["day12"] = [{"text":"impl Send for Action","synthetic":true,"types":[]},{"text":"impl Send for Position","synthetic":true,"types":[]},{"text":"impl Send for Waypoint","synthetic":true,"types":[]}];
implementors["either"] = [{"text":"impl&lt;L, R&gt; Send for Either&lt;L, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;L: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Send,&nbsp;</span>","synthetic":true,"types":[]}];
implementors["itertools"] = [{"text":"impl&lt;I, J&gt; Send for Diff&lt;I, J&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;J: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;I as Iterator&gt;::Item: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;J as Iterator&gt;::Item: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for MinMaxResult&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Position&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;A, B&gt; Send for EitherOrBoth&lt;A, B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for FoldWhile&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;I, Pred&gt; Send for DedupBy&lt;I, Pred&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;Pred: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;I as Iterator&gt;::Item: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;I, J&gt; Send for Interleave&lt;I, J&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;J: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;I, J&gt; Send for InterleaveShortest&lt;I, J&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;J: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;I, J&gt; Send for Product&lt;I, J&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;J: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;I as Iterator&gt;::Item: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;I&gt; Send for PutBack&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;I as Iterator&gt;::Item: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;I, F&gt; Send for Batching&lt;I, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;I, R&gt; Send for MapInto&lt;I, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;I, F&gt; Send for MapResults&lt;I, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;I, J, F&gt; Send for MergeBy&lt;I, J, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;J: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;I as Iterator&gt;::Item: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, I, F&gt; Send for TakeWhileRef&lt;'a, I, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;I&gt; Send for WhileSome&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;I, F&gt; Send for Coalesce&lt;I, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;I as Iterator&gt;::Item: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;I, T&gt; Send for TupleCombinations&lt;I, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as HasCombination&lt;I&gt;&gt;::Combination: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;I, F&gt; Send for Positions&lt;I, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;I, F&gt; Send for Update&lt;I, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;I&gt; Send for Step&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;I&gt; Send for MultiProduct&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;I as Iterator&gt;::Item: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;I&gt; Send for Combinations&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;I as Iterator&gt;::Item: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;I&gt; Send for CombinationsWithReplacement&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;I as Iterator&gt;::Item: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;I, J&gt; Send for ConsTuples&lt;I, J&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;I&gt; Send for ExactlyOneError&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;I as Iterator&gt;::Item: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, I&gt; Send for Format&lt;'a, I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, I, F&gt; Send for FormatWith&lt;'a, I, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;I&gt; Send for IntoChunks&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;I as Iterator&gt;::Item: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, I&gt; !Send for Chunk&lt;'a, I&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, I&gt; !Send for Chunks&lt;'a, I&gt;","synthetic":true,"types":[]},{"text":"impl&lt;K, I, F&gt; Send for GroupBy&lt;K, I, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;K: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;I as Iterator&gt;::Item: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, K, I, F&gt; !Send for Group&lt;'a, K, I, F&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, K, I, F&gt; !Send for Groups&lt;'a, K, I, F&gt;","synthetic":true,"types":[]},{"text":"impl&lt;I&gt; Send for Intersperse&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;I as Iterator&gt;::Item: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;I, F&gt; Send for KMergeBy&lt;I, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;I as Iterator&gt;::Item: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;I, J, F&gt; Send for MergeJoinBy&lt;I, J, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;J: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;I as Iterator&gt;::Item: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;J as Iterator&gt;::Item: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;I&gt; Send for MultiPeek&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;I as Iterator&gt;::Item: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;I, F&gt; Send for PadUsing&lt;I, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, I, F&gt; Send for PeekingTakeWhile&lt;'a, I, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;I&gt; Send for Permutations&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;I as Iterator&gt;::Item: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, I, E&gt; Send for ProcessResults&lt;'a, I, E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;I&gt; Send for PutBackN&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;I as Iterator&gt;::Item: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;I&gt; !Send for RcIter&lt;I&gt;","synthetic":true,"types":[]},{"text":"impl&lt;A&gt; Send for RepeatN&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;F&gt; Send for RepeatCall&lt;F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;St, F&gt; Send for Unfold&lt;St, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;St: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;St, F&gt; Send for Iterate&lt;St, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;St: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;I&gt; !Send for Tee&lt;I&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for TupleBuffer&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as TupleCollect&gt;::Buffer: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;I, T&gt; Send for TupleWindows&lt;I, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;I, T&gt; Send for Tuples&lt;I, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as TupleCollect&gt;::Buffer: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;I&gt; Send for Unique&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;I as Iterator&gt;::Item: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;I, V, F&gt; Send for UniqueBy&lt;I, V, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;V: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;I&gt; Send for WithPosition&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;I as Iterator&gt;::Item: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;I, J&gt; Send for ZipEq&lt;I, J&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;J: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T, U&gt; Send for ZipLongest&lt;T, U&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;U: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Zip&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()