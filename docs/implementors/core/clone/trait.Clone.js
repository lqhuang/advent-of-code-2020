(function() {var implementors = {};
implementors["day11"] = [{"text":"impl Clone for Seat","synthetic":false,"types":[]},{"text":"impl Clone for SeatGrid","synthetic":false,"types":[]},{"text":"impl Clone for FlapType","synthetic":false,"types":[]}];
implementors["either"] = [{"text":"impl&lt;L:&nbsp;Clone, R:&nbsp;Clone&gt; Clone for Either&lt;L, R&gt;","synthetic":false,"types":[]}];
implementors["itertools"] = [{"text":"impl&lt;I:&nbsp;Clone&gt; Clone for MultiProduct&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Iterator + Clone,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::Item: Clone,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;I:&nbsp;Clone, J:&nbsp;Clone&gt; Clone for Interleave&lt;I, J&gt;","synthetic":false,"types":[]},{"text":"impl&lt;I:&nbsp;Clone, J:&nbsp;Clone&gt; Clone for InterleaveShortest&lt;I, J&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Iterator,<br>&nbsp;&nbsp;&nbsp;&nbsp;J: Iterator&lt;Item = I::Item&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;I:&nbsp;Clone&gt; Clone for PutBack&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Iterator,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::Item: Clone,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;I:&nbsp;Clone, J:&nbsp;Clone&gt; Clone for Product&lt;I, J&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Iterator,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::Item: Clone,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;I:&nbsp;Clone, F:&nbsp;Clone&gt; Clone for Batching&lt;I, F&gt;","synthetic":false,"types":[]},{"text":"impl&lt;I:&nbsp;Clone&gt; Clone for Step&lt;I&gt;","synthetic":false,"types":[]},{"text":"impl&lt;I, J, F&gt; Clone for MergeBy&lt;I, J, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Iterator,<br>&nbsp;&nbsp;&nbsp;&nbsp;J: Iterator&lt;Item = I::Item&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Peekable&lt;I&gt;: Clone,<br>&nbsp;&nbsp;&nbsp;&nbsp;Peekable&lt;J&gt;: Clone,<br>&nbsp;&nbsp;&nbsp;&nbsp;F: Clone,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;I:&nbsp;Clone, F:&nbsp;Clone&gt; Clone for Coalesce&lt;I, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Iterator,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::Item: Clone,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;I:&nbsp;Clone, Pred:&nbsp;Clone&gt; Clone for DedupBy&lt;I, Pred&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Iterator,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::Item: Clone,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;I:&nbsp;Clone&gt; Clone for WhileSome&lt;I&gt;","synthetic":false,"types":[]},{"text":"impl&lt;I:&nbsp;Clone, T:&nbsp;Clone&gt; Clone for TupleCombinations&lt;I, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Iterator,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: HasCombination&lt;I&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T::Combination: Clone,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;I:&nbsp;Clone, R:&nbsp;Clone&gt; Clone for MapInto&lt;I, R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;I:&nbsp;Clone, F:&nbsp;Clone&gt; Clone for MapResults&lt;I, F&gt;","synthetic":false,"types":[]},{"text":"impl&lt;I:&nbsp;Clone, F:&nbsp;Clone&gt; Clone for Positions&lt;I, F&gt;","synthetic":false,"types":[]},{"text":"impl&lt;I:&nbsp;Clone, F:&nbsp;Clone&gt; Clone for Update&lt;I, F&gt;","synthetic":false,"types":[]},{"text":"impl&lt;A:&nbsp;Clone, B:&nbsp;Clone&gt; Clone for EitherOrBoth&lt;A, B&gt;","synthetic":false,"types":[]},{"text":"impl&lt;I, J&gt; Clone for ConsTuples&lt;I, J&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Clone + Iterator&lt;Item = J&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;I&gt; Clone for Combinations&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Clone + Iterator,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::Item: Clone,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;I:&nbsp;Clone&gt; Clone for CombinationsWithReplacement&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Iterator,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::Item: Clone,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;I:&nbsp;Clone&gt; Clone for ExactlyOneError&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Iterator,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::Item: Clone,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::Item: Clone,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, I:&nbsp;Clone, F:&nbsp;Clone&gt; Clone for FormatWith&lt;'a, I, F&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, I:&nbsp;Clone&gt; Clone for Format&lt;'a, I&gt;","synthetic":false,"types":[]},{"text":"impl&lt;I:&nbsp;Clone&gt; Clone for Intersperse&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Iterator,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::Item: Clone,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::Item: Clone,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;I, F&gt; Clone for KMergeBy&lt;I, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Iterator + Clone,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::Item: Clone,<br>&nbsp;&nbsp;&nbsp;&nbsp;F: Clone,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;I, J, F&gt; Clone for MergeJoinBy&lt;I, J, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Iterator,<br>&nbsp;&nbsp;&nbsp;&nbsp;J: Iterator,<br>&nbsp;&nbsp;&nbsp;&nbsp;PutBack&lt;Fuse&lt;I&gt;&gt;: Clone,<br>&nbsp;&nbsp;&nbsp;&nbsp;PutBack&lt;Fuse&lt;J&gt;&gt;: Clone,<br>&nbsp;&nbsp;&nbsp;&nbsp;F: Clone,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Clone&gt; Clone for MinMaxResult&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;I:&nbsp;Clone&gt; Clone for MultiPeek&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Iterator,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::Item: Clone,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;I:&nbsp;Clone, F:&nbsp;Clone&gt; Clone for PadUsing&lt;I, F&gt;","synthetic":false,"types":[]},{"text":"impl&lt;I&gt; Clone for Permutations&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Clone + Iterator,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::Item: Clone,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;I:&nbsp;Clone + Iterator&gt; Clone for PutBackN&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I::Item: Clone,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;I&gt; Clone for RcIter&lt;I&gt;","synthetic":false,"types":[]},{"text":"impl&lt;A:&nbsp;Clone&gt; Clone for RepeatN&lt;A&gt;","synthetic":false,"types":[]},{"text":"impl&lt;F:&nbsp;Clone&gt; Clone for RepeatCall&lt;F&gt;","synthetic":false,"types":[]},{"text":"impl&lt;St:&nbsp;Clone, F:&nbsp;Clone&gt; Clone for Unfold&lt;St, F&gt;","synthetic":false,"types":[]},{"text":"impl&lt;St:&nbsp;Clone, F:&nbsp;Clone&gt; Clone for Iterate&lt;St, F&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Clone&gt; Clone for TupleBuffer&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: HomogeneousTuple,<br>&nbsp;&nbsp;&nbsp;&nbsp;T::Buffer: Clone,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;I:&nbsp;Clone, T:&nbsp;Clone&gt; Clone for Tuples&lt;I, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Iterator&lt;Item = T::Item&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: HomogeneousTuple,<br>&nbsp;&nbsp;&nbsp;&nbsp;T::Buffer: Clone,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;I:&nbsp;Clone, T:&nbsp;Clone&gt; Clone for TupleWindows&lt;I, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Iterator&lt;Item = T::Item&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: HomogeneousTuple,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;I:&nbsp;Clone + Iterator, V:&nbsp;Clone, F:&nbsp;Clone&gt; Clone for UniqueBy&lt;I, V, F&gt;","synthetic":false,"types":[]},{"text":"impl&lt;I:&nbsp;Clone + Iterator&gt; Clone for Unique&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I::Item: Clone,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;I&gt; Clone for WithPosition&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Clone + Iterator,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::Item: Clone,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Clone&gt; Clone for Position&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;I:&nbsp;Clone, J:&nbsp;Clone&gt; Clone for ZipEq&lt;I, J&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Clone, U:&nbsp;Clone&gt; Clone for ZipLongest&lt;T, U&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Clone&gt; Clone for Zip&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Clone&gt; Clone for FoldWhile&lt;T&gt;","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()